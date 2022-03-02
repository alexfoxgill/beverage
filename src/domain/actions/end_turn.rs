use crate::{
    domain::effects::end_turn::EndTurnEffect,
    turn_engine::{
        actions::{Action, ActionQueue, ActionResult},
        effects::EffectQueue,
    },
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct EndTurnAction(Entity);

impl EndTurnAction {
    pub fn new(entity: Entity) -> EndTurnAction {
        EndTurnAction(entity)
    }
}

impl Action for EndTurnAction {
    fn cost(&self) -> u8 {
        0
    }
}

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::new(EndTurnAction::new(e))
}

pub fn handler(In(EndTurnAction(entity)): In<EndTurnAction>) -> ActionResult {
    Ok(EffectQueue::new(EndTurnEffect::new(entity)))
}
