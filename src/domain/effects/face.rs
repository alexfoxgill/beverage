use bevy::prelude::*;

use crate::domain::common::{Facing, HexDirection};

use crate::turn_engine::effects::Effect;

#[derive(Debug)]
pub struct FaceEffect(Entity, HexDirection);

impl FaceEffect {
    pub fn new(entity: Entity, face: HexDirection) -> FaceEffect {
        FaceEffect(entity, face)
    }
}

impl Effect for FaceEffect {}

pub fn handler(In(FaceEffect(entity, face)): In<FaceEffect>, mut facings: Query<&mut Facing>) {
    if let Ok(mut facing) = facings.get_mut(entity) {
        facing.0 = face;
    }
}
