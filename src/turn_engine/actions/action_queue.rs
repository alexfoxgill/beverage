use std::{any::TypeId, collections::VecDeque};

use super::{Action, ActionEvent};
use bevy::{prelude::*, utils::HashMap};

#[derive(Default)]
pub struct ActionQueue(VecDeque<ActionEvent>);

impl ActionQueue {
    pub fn pop(&mut self) -> Option<ActionEvent> {
        self.0.pop_front()
    }

    pub fn push(&mut self, action: ActionEvent) {
        self.0.push_back(action);
    }
}

#[derive(Default)]
pub struct ActionSchedules(pub HashMap<TypeId, Schedule>);

impl ActionSchedules {
    pub fn register_handler<T: Action + 'static>(&mut self, schedule: Schedule) {
        self.0.insert(TypeId::of::<T>(), schedule);
    }
}

pub struct CurrentAction<T>(pub T);

pub struct ActionDispatcherStage;

impl Stage for ActionDispatcherStage {
    fn run(&mut self, world: &mut World) {
        world.resource_scope(|world, mut schedules: Mut<ActionSchedules>| loop {
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
