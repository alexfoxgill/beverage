use crate::{
    common::*,
    domain::effects::face_effect::FaceEffect,
    turn_engine::{actions::Action, effects::EffectQueue, Handled, TurnSchedules},
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

impl Action for RotateAction {
    fn insert_handled(self: Box<Self>, world: &mut World) {
        world.insert_resource(Handled(*self));
    }
}

pub struct RotateActionPlugin;

impl Plugin for RotateActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    schedules.register_action_system(handler.system())
}

fn handler(action: In<RotateAction>, mut effect_queue: ResMut<EffectQueue>) {
    effect_queue.push(FaceEffect::new(action.0.entity, action.0.to));
}
