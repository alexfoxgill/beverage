use bevy::prelude::*;
use hex2d::Coordinate;

use crate::{
    common::HexPos,
    turn_engine::{effects::Effect, TurnSystems},
};

#[derive(Debug)]
pub struct MoveEffect(Entity, Coordinate);

impl MoveEffect {
    pub fn new(entity: Entity, to: Coordinate) -> MoveEffect {
        MoveEffect(entity, to)
    }
}

impl Effect for MoveEffect {}

pub struct MoveEffectPlugin;

impl Plugin for MoveEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_effect_handler(handler.system());
}

fn handler(In(MoveEffect(entity, to)): In<MoveEffect>, mut positions: Query<&mut HexPos>) {
    if let Ok(mut pos) = positions.get_mut(entity) {
        pos.0 = to;
    }
}
