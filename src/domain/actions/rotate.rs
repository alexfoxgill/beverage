use crate::{
    common::*,
    domain::effects::face::FaceEffect,
    turn_engine::{actions::Action, effects::EffectQueue, TurnSystems},
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

pub struct RotateActionPlugin;

impl Plugin for RotateActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_action_handler(handler.system())
}

fn handler(
    In(RotateAction(entity, by)): In<RotateAction>,
    query: Query<&Facing>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok(facing) = query.get(entity) {
        let target = facing.rotated(by);
        effect_queue.push(FaceEffect::new(entity, target));
    }
}
