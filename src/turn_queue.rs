use std::collections::VecDeque;

use super::common::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct TurnQueue {
    pub queue: VecDeque<Entity>,
}

pub fn cycle_turn_queue(mut actors: Query<&mut Actor>, mut turn_queue: ResMut<TurnQueue>) {
    if let Some(&entity) = turn_queue.queue.front() {
        if let Ok(mut actor) = actors.get_mut(entity) {
            if actor.actions_remaining <= 0 {
                actor.actions_remaining = actor.actions_per_turn;
                turn_queue.queue.pop_front();
                turn_queue.queue.push_back(entity);
            }
        } else {
            turn_queue.queue.pop_front();
        }
    }
}
