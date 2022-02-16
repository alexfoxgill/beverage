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

trait AnyRunner<T, Out = ()>: Send + Sync {
    fn run(&mut self, input: T, world: &mut World) -> Out;
}

impl<A: Action, S> AnyRunner<AnyAction, EffectQueue> for TypedSystemRunner<A, EffectQueue, S>
where
    S: System<In = A, Out = EffectQueue>,
{
    fn run(&mut self, action: AnyAction, world: &mut World) -> EffectQueue {
        if let Ok(action) = action.0.downcast::<A>() {
            if !self.initialized {
                self.system.initialize(world);
                self.initialized = true;
            }
            let effects = self.system.run(*action, world);
            self.system.apply_buffers(world);
            effects
        } else {
            EffectQueue::default()
        }
    }
}

impl<E: Effect, S> AnyRunner<AnyEffect> for TypedSystemRunner<E, (), S>
where
    S: System<In = E, Out = ()>,
{
    fn run(&mut self, effect: AnyEffect, world: &mut World) {
        if let Ok(effect) = effect.0.downcast::<E>() {
            if !self.initialized {
                self.system.initialize(world);
                self.initialized = true;
            }
            self.system.run(*effect, world);
            self.system.apply_buffers(world);
        }
    }
}

#[derive(Default)]
pub struct TurnSystems {
    effects: HashMap<TypeId, Box<dyn AnyRunner<AnyEffect>>>,
    actions: HashMap<TypeId, Box<dyn AnyRunner<AnyAction, EffectQueue>>>,
}

impl TurnSystems {
    pub fn register_action_handler<A: Action + 'static>(
        &mut self,
        system: impl System<In = A, Out = EffectQueue>,
    ) {
        self.actions
            .insert(TypeId::of::<A>(), Box::new(TypedSystemRunner::new(system)));
    }

    pub fn run_action_system(&mut self, action: AnyAction, world: &mut World) -> EffectQueue {
        let action_type = action.inner_type();
        if let Some(handler) = self.actions.get_mut(&action_type) {
            handler.run(action, world)
        } else {
            eprintln!("Could not find scheduler for action {:?}", action_type);
            EffectQueue::default()
        }
    }

    pub fn register_effect_handler<E: Effect + 'static>(
        &mut self,
        system: impl System<In = E, Out = ()>,
    ) {
        self.effects
            .insert(TypeId::of::<E>(), Box::new(TypedSystemRunner::new(system)));
    }

    pub fn run_effect_system(&mut self, effect: AnyEffect, world: &mut World) {
        let effect_type = effect.inner_type();
        if let Some(handler) = self.effects.get_mut(&effect_type) {
            handler.run(effect, world);
        } else {
            eprintln!("Could not find scheduler for effect {:?}", effect_type);
        }
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
