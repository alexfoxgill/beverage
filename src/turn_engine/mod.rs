use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};

use self::{
    actions::{Action, ActionQueue},
    effects::{Effect, EffectQueue},
};

pub mod actions;
pub mod effects;

#[derive(Default)]
pub struct TurnSchedules {
    actions: HashMap<TypeId, Schedule>,
    effects: HashMap<TypeId, Schedule>,
}

impl TurnSchedules {
    pub fn register_action_handler<T: Action + 'static>(&mut self, schedule: Schedule) {
        self.actions.insert(TypeId::of::<T>(), schedule);
    }

    pub fn register_effect_handler<T: Effect + 'static>(&mut self, schedule: Schedule) {
        self.effects.insert(TypeId::of::<T>(), schedule);
    }
}

pub struct Handled<T>(pub T);

pub struct TurnExecutorLoop;

impl Stage for TurnExecutorLoop {
    fn run(&mut self, world: &mut World) {
        world.resource_scope(|world, mut schedules: Mut<TurnSchedules>| 'actions: loop {
            let mut action_queue = world.get_resource_mut::<ActionQueue>().unwrap();
            if let Some(action) = action_queue.0.pop_front() {
                let action_type = action.inner_type();
                if let Some(action_schedule) = schedules.actions.get_mut(&action_type) {
                    action.insert_resource(world);
                    action_schedule.run(world);
                } else {
                    eprintln!("Could not find scheduler for action {:?}", action_type);
                    continue 'actions;
                }

                'effects: loop {
                    let mut effect_queue = world.get_resource_mut::<EffectQueue>().unwrap();
                    if let Some(effect) = effect_queue.0.pop_front() {
                        let effect_type = effect.inner_type();
                        if let Some(effect_schedule) = schedules.effects.get_mut(&effect_type) {
                            effect.insert_resource(world);
                            effect_schedule.run(world);
                        } else {
                            eprintln!("Could not find scheduler for effect {:?}", effect_type);
                            continue 'effects;
                        }
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
        app.init_resource::<TurnSchedules>()
            .init_resource::<ActionQueue>()
            .init_resource::<EffectQueue>()
            .add_stage_after(CoreStage::Update, TurnExecution, TurnExecutorLoop);
    }
}
