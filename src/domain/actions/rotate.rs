use crate::{
    common::*,
    domain::effects::face::FaceEffect,
    turn_engine::{actions::Action, effects::EffectQueue},
};
use bevy::prelude::*;
use hex2d::Angle;

#[derive(Debug)]
pub struct RotateAction {
    entity: Entity,
    angle: Angle,
}

impl RotateAction {
    pub fn new(entity: Entity, angle: Angle) -> RotateAction {
        RotateAction { entity, angle }
    }
}

impl Action for RotateAction {}

pub fn handler(In(action): In<RotateAction>, query: Query<&Facing>) -> EffectQueue {
    let mut effects = EffectQueue::default();
    if let Ok(facing) = query.get(action.entity) {
        let target = facing.rotated(action.angle);
        effects.push(FaceEffect::new(action.entity, target));
    }
    effects
}
