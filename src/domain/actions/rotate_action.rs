use crate::{
    common::*,
    domain::effects::face_effect::FaceEffect,
    turn_engine::{
        action_queue::{ActionSchedules, CurrentAction},
        actions::{Action, ActionEvent},
        effects::EffectEvent,
    },
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct RotateAction {
    entity: Entity,
    to: HexDirection,
}

impl RotateAction {
    pub fn event(entity: Entity, to: HexDirection) -> ActionEvent {
        ActionEvent(Box::new(RotateAction { entity, to }))
    }
}

impl Action for RotateAction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn insert_resource(&self, world: &mut World) {
        let action = CurrentAction(self.clone());
        world.insert_resource(action)
    }
}

pub struct RotateActionPlugin;

impl Plugin for RotateActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<ActionSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_handler::<RotateAction>(schedule)
}

fn handler(action: Res<CurrentAction<RotateAction>>, mut effects: EventWriter<EffectEvent>) {
    effects.send(FaceEffect::event(action.0.entity, action.0.to));
}
