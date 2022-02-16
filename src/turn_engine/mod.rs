use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};

use self::{
    actions::{Action, ActionQueue, AnyAction},
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
    Out: Default,
{
    fn run(&mut self, action: InDyn, world: &mut World) -> Out {
        if let Some(input) = action.downcast() {
            if !self.initialized {
                self.system.initialize(world);
                self.initialized = true;
            }
            let res = self.system.run(input, world);
            self.system.apply_buffers(world);
            res
        } else {
            Default::default()
        }
    }
}

struct SystemRegistry<InDyn, Out> {
    pub map: HashMap<TypeId, Box<dyn AnyRunner<InDyn, Out>>>,
}

impl<InDyn, Out> Default for SystemRegistry<InDyn, Out> {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl<InDyn: InnerType, Out: Default> AnyRunner<InDyn, Out> for SystemRegistry<InDyn, Out> {
    fn run(&mut self, input: InDyn, world: &mut World) -> Out {
        let input_type = input.inner_type();
        if let Some(handler) = self.map.get_mut(&input_type) {
            handler.run(input, world)
        } else {
            eprintln!("Could not find runner for {:?}", input_type);
            Default::default()
        }
    }
}

#[derive(Default)]
pub struct TurnSystems {
    effects: SystemRegistry<AnyEffect, ()>,
    actions: SystemRegistry<AnyAction, EffectQueue>,
}

impl TurnSystems {
    pub fn register_action_handler<A: Action + 'static>(
        &mut self,
        system: impl System<In = A, Out = EffectQueue>,
    ) {
        self.actions
            .map
            .insert(TypeId::of::<A>(), Box::new(TypedSystemRunner::new(system)));
    }

    pub fn run_action_system(&mut self, action: AnyAction, world: &mut World) -> EffectQueue {
        self.actions.run(action, world)
    }

    pub fn register_effect_handler<E: Effect + 'static>(
        &mut self,
        system: impl System<In = E, Out = ()>,
    ) {
        self.effects
            .map
            .insert(TypeId::of::<E>(), Box::new(TypedSystemRunner::new(system)));
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
            if let Some(action) = action_queue.0.pop_front() {
                let new_effects = systems.run_action_system(action, world);
                let mut effect_queue = world.get_resource_mut::<EffectQueue>().unwrap();
                effect_queue.append(new_effects);

                'effects: loop {
                    let mut effect_queue = world.get_resource_mut::<EffectQueue>().unwrap();
                    if let Some(effect) = effect_queue.0.pop_front() {
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
