use crate::{
    domain::effects::end_turn::EndTurnEffect,
    turn_engine::{actions::Action, effects::EffectQueue},
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

pub fn handler(In(EndTurnAction(entity)): In<EndTurnAction>, mut effects: ResMut<EffectQueue>) {
    effects.push(EndTurnEffect::new(entity));
}
