use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};
use bevy_ecs::archetype::ArchetypeGeneration;

use self::{
    actions::{Action, ActionQueue, ActionResult, AnyAction},
    effects::{AnyEffect, Effect, EffectQueue},
};

pub mod actions;
pub mod effects;

struct TypedSystemRunner<In, Out, S>
where
    S: System<In = In, Out = Out>,
{
    system: S,
    initialized: bool,
    archetype_generation: ArchetypeGeneration,
}

impl<In, Out, S> TypedSystemRunner<In, Out, S>
where
    S: System<In = In, Out = Out>,
{
    pub fn new(system: S) -> TypedSystemRunner<In, Out, S> {
        TypedSystemRunner {
            system,
            initialized: false,
            archetype_generation: ArchetypeGeneration::initial(),
        }
    }
}

trait AnyRunner<InDyn, Out = ()>: Send + Sync {
    fn run(&mut self, input: InDyn, world: &mut World) -> Out;
}

trait DynamicWrapper<T> {
    fn downcast(self) -> Option<T>;
}
trait InnerType {
    fn inner_type(&self) -> TypeId;
}

impl<InDyn, In, Out, S> AnyRunner<InDyn, Out> for TypedSystemRunner<In, Out, S>
where
    InDyn: DynamicWrapper<In>,
    S: System<In = In, Out = Out>,
{
    fn run(&mut self, input: InDyn, world: &mut World) -> Out {
        if let Some(input) = input.downcast() {
            if !self.initialized {
                self.system.initialize(world);
                self.initialized = true;
            } else {
                let archetypes = world.archetypes();
                let new_generation = archetypes.generation();
                let old_generation =
                    std::mem::replace(&mut self.archetype_generation, new_generation);

                let new_archetype_count = new_generation.value() - old_generation.value();

                if new_archetype_count > 0 {
                    for archetype in archetypes
                        .iter()
                        .skip(old_generation.value())
                        .take(new_archetype_count)
                    {
                        self.system.new_archetype(archetype);
                    }
                }
            }
            let res = self.system.run(input, world);
            self.system.apply_buffers(world);
            res
        } else {
            panic!("AnyRunner downcast failed");
        }
    }
}

struct SystemRegistry<InDyn, Out = ()> {
    map: HashMap<TypeId, Box<dyn AnyRunner<InDyn, Out>>>,
}

impl<InDyn, Out: 'static> SystemRegistry<InDyn, Out> {
    pub fn register_system<In: 'static, Params>(&mut self, system: impl IntoSystem<In, Out, Params>)
    where
        InDyn: DynamicWrapper<In>,
    {
        self.map.insert(
            TypeId::of::<In>(),
            Box::new(TypedSystemRunner::new(system.system())),
        );
    }
}

impl<InDyn, Out> Default for SystemRegistry<InDyn, Out> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<InDyn: InnerType, Out> AnyRunner<InDyn, Out> for SystemRegistry<InDyn, Out> {
    fn run(&mut self, input: InDyn, world: &mut World) -> Out {
        let input_type = input.inner_type();
        if let Some(handler) = self.map.get_mut(&input_type) {
            handler.run(input, world)
        } else {
            panic!("Could not find runner for {:?}", input_type);
        }
    }
}

#[derive(Default)]
pub struct TurnSystems {
    effects: SystemRegistry<AnyEffect>,
    actions: SystemRegistry<AnyAction, ActionResult>,
}

impl TurnSystems {
    pub fn register_action_handler<A: Action + 'static, Params>(
        &mut self,
        system: impl IntoSystem<A, ActionResult, Params>,
    ) {
        self.actions.register_system(system);
    }

    pub fn run_action_system(&mut self, action: AnyAction, world: &mut World) -> ActionResult {
        self.actions.run(action, world)
    }

    pub fn register_effect_handler<E: Effect + 'static, Params>(
        &mut self,
        system: impl IntoSystem<E, (), Params>,
    ) {
        self.effects.register_system(system);
    }

    pub fn run_effect_system(&mut self, effect: AnyEffect, world: &mut World) {
        self.effects.run(effect, world)
    }
}

pub struct ActionExecutor;
impl Stage for ActionExecutor {
    fn run(&mut self, world: &mut World) {
        world.resource_scope(|world, mut state: Mut<TurnState>| {
            if let TurnState::Idle = *state {
                world.resource_scope(|world, mut systems: Mut<TurnSystems>| {
                    let mut action_queue = world.get_resource_mut::<ActionQueue>().unwrap();
                    if let Some(action) = action_queue.pop() {
                        match systems.run_action_system(action.clone(), world) {
                            Ok(effects) => {
                                *state = TurnState::Executing { action, effects };
                            }
                            Err(message) => {
                                eprintln!("Action forbidden: {message:?}")
                            }
                        }
                    }
                });
            }
        });
    }
}

pub struct EffectExecutor;
impl Stage for EffectExecutor {
    fn run(&mut self, world: &mut World) {
        world.resource_scope(|world, mut state: Mut<TurnState>| {
            if let TurnState::Executing { effects, .. } = state.as_mut() {
                world.resource_scope(|world, mut systems: Mut<TurnSystems>| loop {
                    if let Some(effect) = effects.pop() {
                        systems.run_effect_system(effect, world);
                    } else {
                        break;
                    }
                });

                *state = TurnState::Idle;
            }
        });
    }
}

pub enum TurnState {
    Idle,
    Executing {
        action: AnyAction,
        effects: EffectQueue,
    },
    Paused,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, StageLabel)]
pub enum TurnStage {
    Action,
    Effects,
}

pub struct TurnEnginePlugin;

impl Plugin for TurnEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnSystems>()
            .init_resource::<ActionQueue>()
            .insert_resource(TurnState::Idle)
            .add_stage_after(CoreStage::Update, TurnStage::Action, ActionExecutor)
            .add_stage_after(TurnStage::Action, TurnStage::Effects, EffectExecutor);
    }
}
