use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};

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
}

impl<In, Out, S> TypedSystemRunner<In, Out, S>
where
    S: System<In = In, Out = Out>,
{
    pub fn new(system: S) -> TypedSystemRunner<In, Out, S> {
        TypedSystemRunner {
            system,
            initialized: false,
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

pub struct TurnExecutorLoop;

impl Stage for TurnExecutorLoop {
    fn run(&mut self, world: &mut World) {
        world.resource_scope(|world, mut systems: Mut<TurnSystems>| 'actions: loop {
            let mut action_queue = world.get_resource_mut::<ActionQueue>().unwrap();
            if let Some(action) = action_queue.pop() {
                match systems.run_action_system(action, world) {
                    Ok(new_effects) => {
                        let mut effect_queue = world.get_resource_mut::<EffectQueue>().unwrap();
                        effect_queue.append(new_effects);
                    }
                    Err(message) => {
                        eprintln!("Action forbidden: {message:?}")
                    }
                }

                'effects: loop {
                    let mut effect_queue = world.get_resource_mut::<EffectQueue>().unwrap();
                    if let Some(effect) = effect_queue.pop() {
                        systems.run_effect_system(effect, world);
                    } else {
                        break 'effects;
                    }
                }
            } else {
                break 'actions;
            }
        });
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, StageLabel)]
pub struct TurnExecution;

pub struct TurnEnginePlugin;

impl Plugin for TurnEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnSystems>()
            .init_resource::<ActionQueue>()
            .init_resource::<EffectQueue>()
            .add_stage_after(CoreStage::Update, TurnExecution, TurnExecutorLoop);
    }
}
