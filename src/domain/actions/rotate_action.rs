use crate::{
    common::*,
    domain::effects::face_effect::FaceEffect,
    turn_engine::{actions::Action, effects::EffectQueue, Handled, TurnSchedules},
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
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
    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct RotateActionPlugin;

impl Plugin for RotateActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_action_handler::<RotateAction>(schedule)
}

fn handler(action: Res<Handled<RotateAction>>, mut effect_queue: ResMut<EffectQueue>) {
    effect_queue.push(FaceEffect::new(action.0.entity, action.0.to));
}
