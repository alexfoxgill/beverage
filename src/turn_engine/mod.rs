use std::any::TypeId;

use bevy::{prelude::*, utils::HashMap};

use self::{
    actions::{Action, ActionQueue},
    effects::EffectEvent,
};

pub mod actions;
pub mod effects;

#[derive(Default)]
pub struct TurnSchedules(HashMap<TypeId, Schedule>);

impl TurnSchedules {
    pub fn register_action_handler<T: Action + 'static>(&mut self, schedule: Schedule) {
        self.0.insert(TypeId::of::<T>(), schedule);
    }
}

pub struct Handled<T>(pub T);

pub struct ActionDispatcherStage;

impl Stage for ActionDispatcherStage {
    fn run(&mut self, world: &mut World) {
        world.resource_scope(|world, mut schedules: Mut<TurnSchedules>| loop {
            let mut action_queue = world.get_resource_mut::<ActionQueue>().unwrap();
            if let Some(action) = action_queue.0.pop_front() {
                let type_id = action.inner_type();
                if let Some(schedule) = schedules.0.get_mut(&type_id) {
                    action.insert_resource(world);
                    schedule.run(world);
                } else {
                    println!("Could not find scheduler for {:?}", type_id);
                }
            } else {
                break;
            }
        });
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, StageLabel)]
pub struct ActionDispatcher;

#[derive(Debug, Clone, Hash, PartialEq, Eq, StageLabel)]
pub struct EffectDispatcher;

pub struct TurnEnginePlugin;

impl Plugin for TurnEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnSchedules>()
            .init_resource::<ActionQueue>()
            .add_stage_after(CoreStage::Update, ActionDispatcher, ActionDispatcherStage)
            .add_event::<EffectEvent>()
            .add_stage_after(ActionDispatcher, EffectDispatcher, SystemStage::parallel());
    }
}
