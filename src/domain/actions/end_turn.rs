use crate::{
    domain::effects::end_turn::EndTurnEffect,
    turn_engine::{
        actions::{Action, ActionQueue},
        effects::EffectQueue,
    },
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct EndTurnAction(Entity);

impl EndTurnAction {
    pub fn new(entity: Entity) -> EndTurnAction {
        EndTurnAction(entity)
    }
}

impl Action for EndTurnAction {}

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::new(EndTurnAction::new(e))
}

pub fn handler(In(EndTurnAction(entity)): In<EndTurnAction>) -> EffectQueue {
    EffectQueue::new(EndTurnEffect::new(entity))
}
