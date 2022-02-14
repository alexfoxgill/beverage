use bevy::prelude::*;
use hex2d::Coordinate;

use crate::{
    common::HexPos,
    turn_engine::{effects::Effect, TurnSchedules},
};

#[derive(Debug)]
pub struct MoveEffect {
    entity: Entity,
    to: Coordinate,
}

impl MoveEffect {
    pub fn new(entity: Entity, to: Coordinate) -> MoveEffect {
        MoveEffect { entity, to }
    }
}

impl Effect for MoveEffect {}

pub struct MoveEffectPlugin;

impl Plugin for MoveEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    schedules.register_effect_handler(handler.system());
}

fn handler(effect: In<MoveEffect>, mut positions: Query<&mut HexPos>) {
    if let Ok(mut pos) = positions.get_mut(effect.0.entity) {
        pos.0 = effect.0.to;
    }
}
