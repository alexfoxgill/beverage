use crate::turn_engine::{effects::Effect, TurnSystems};

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

pub struct EndTurnEffectPlugin;

impl Plugin for EndTurnEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_effect_handler(handler.system());
}

fn handler(
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
