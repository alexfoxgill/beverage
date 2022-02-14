use crate::{
    common::*,
    domain::effects::face_effect::FaceEffect,
    turn_engine::{actions::Action, effects::EffectQueue, TurnSystems},
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct RotateAction {
    entity: Entity,
    to: HexDirection,
}

impl RotateAction {
    pub fn new(entity: Entity, to: HexDirection) -> RotateAction {
        RotateAction { entity, to }
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

fn handler(In(action): In<RotateAction>, mut effect_queue: ResMut<EffectQueue>) {
    effect_queue.push(FaceEffect::new(action.entity, action.to));
}
