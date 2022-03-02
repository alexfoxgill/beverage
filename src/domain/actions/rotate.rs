use crate::{
    domain::common::*,
    domain::effects::face::FaceEffect,
    turn_engine::{
        actions::{Action, ActionQueue, ActionResult},
        effects::EffectQueue,
    },
};
use bevy::prelude::*;
use hex2d::Angle;

#[derive(Debug, Clone)]
pub struct RotateAction {
    entity: Entity,
    angle: Angle,
}

impl RotateAction {
    pub fn new(entity: Entity, angle: Angle) -> RotateAction {
        RotateAction { entity, angle }
    }
}

impl Action for RotateAction {
    fn cost(&self) -> u8 {
        0
    }
}

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::from([
        RotateAction::new(e, Angle::Left),
        RotateAction::new(e, Angle::Right),
    ])
}

pub fn handler(In(action): In<RotateAction>, query: Query<&Facing>) -> ActionResult {
    let facing = query.get(action.entity)?;
    let target = facing.rotated(action.angle);
    return Ok(EffectQueue::new(FaceEffect::new(action.entity, target)));
}
