use crate::{
    common::*,
    domain::effects::face::FaceEffect,
    turn_engine::{actions::Action, effects::EffectQueue},
};
use bevy::prelude::*;
use hex2d::Angle;

#[derive(Debug)]
pub struct RotateAction(Entity, Angle);

impl RotateAction {
    pub fn new(entity: Entity, by: Angle) -> RotateAction {
        RotateAction(entity, by)
    }
}

impl Action for RotateAction {}

pub fn handler(
    In(RotateAction(entity, by)): In<RotateAction>,
    query: Query<&Facing>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok(facing) = query.get(entity) {
        let target = facing.rotated(by);
        effect_queue.push(FaceEffect::new(entity, target));
    }
}
