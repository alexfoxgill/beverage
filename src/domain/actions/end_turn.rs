use crate::{
    domain::effects::end_turn::EndTurnEffect,
    turn_engine::{actions::Action, effects::EffectQueue, TurnSystems},
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

pub struct EndTurnActionPlugin;

impl Plugin for EndTurnActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_action_handler(handler.system())
}

fn handler(In(EndTurnAction(entity)): In<EndTurnAction>, mut effects: ResMut<EffectQueue>) {
    effects.push(EndTurnEffect::new(entity));
}
