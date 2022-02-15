use crate::turn_engine::effects::Effect;

use crate::common::*;
use crate::turn_queue::TurnQueue;
use bevy::prelude::*;

#[derive(Debug)]
pub struct EndTurnEffect(Entity);

impl EndTurnEffect {
    pub fn new(entity: Entity) -> EndTurnEffect {
        EndTurnEffect(entity)
    }
}

impl Effect for EndTurnEffect {}

pub fn handler(
    In(EndTurnEffect(entity)): In<EndTurnEffect>,
    mut actors: Query<&mut Actor>,
    mut turn_queue: ResMut<TurnQueue>,
) {
    if turn_queue.is_first(entity) {
        if let Ok(mut actor) = actors.get_mut(entity) {
            actor.actions_remaining = actor.actions_per_turn;
            turn_queue.cycle();
        } else {
            turn_queue.remove(entity);
        }
    }
}
