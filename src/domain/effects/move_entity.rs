use bevy::prelude::*;
use hex2d::Coordinate;

use crate::{domain::common::HexPos, turn_engine::effects::Effect};

#[derive(Debug)]
pub struct MoveEffect(Entity, Coordinate);

impl MoveEffect {
    pub fn new(entity: Entity, to: Coordinate) -> MoveEffect {
        MoveEffect(entity, to)
    }
}

impl Effect for MoveEffect {}

pub fn handler(In(MoveEffect(entity, to)): In<MoveEffect>, mut positions: Query<&mut HexPos>) {
    if let Ok(mut pos) = positions.get_mut(entity) {
        pos.0 = to;
    }
}
