use std::collections::VecDeque;

use crate::turn_engine::TurnStage;

use super::common::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct TurnQueue {
    queue: VecDeque<Entity>,
}

impl TurnQueue {
    pub fn head(&self) -> Option<&Entity> {
        self.queue.front()
    }

    pub fn is_first(&self, entity: Entity) -> bool {
        self.head() == Some(&entity)
    }

    pub fn cycle(&mut self) {
        if let Some(entity) = self.queue.pop_front() {
            self.queue.push_back(entity);
        }
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(idx) = self.queue.iter().position(|&e| e == entity) {
            self.queue.remove(idx);
        }
    }

    pub fn enqueue(&mut self, entity: Entity) {
        self.queue.push_back(entity);
    }
}

fn remove_dead_from_queue(dead: RemovedComponents<Actor>, mut turn_queue: ResMut<TurnQueue>) {
    for entity in dead.iter() {
        turn_queue.remove(entity);
    }
}

pub struct TurnQueuePlugin;

impl Plugin for TurnQueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnQueue>().add_stage_after(
            TurnStage::ExecuteEffects,
            "clean_turn_queue",
            SystemStage::parallel().with_system(remove_dead_from_queue),
        );
    }
}
