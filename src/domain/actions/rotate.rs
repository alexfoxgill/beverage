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

    pub fn effects(&self, facing: &Facing, _actor: &Actor) -> EffectQueue {
        let mut queue = EffectQueue::default();
        let target = facing.rotated(self.angle);
        queue.push(FaceEffect::new(self.entity, target));
        queue
    }
}

impl Action for RotateAction {}

pub fn handler(
    In(action): In<RotateAction>,
    query: Query<(&Facing, &Actor)>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok((facing, actor)) = query.get(action.entity) {
        let effects = action.effects(facing, actor);
        effect_queue.append(effects);
    }
}
