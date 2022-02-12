use super::*;
use crate::effects::{
    energy_cost_effect::{ActionCost, EnergyCostEffect},
    face_effect::FaceEffect,
    EffectProducer,
};
use bevy::prelude::*;

#[derive(Debug)]
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

    fn entity(&self) -> Entity {
        self.entity
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![
            EnergyCostEffect::event(self.entity, ActionCost::None),
            FaceEffect::event(self.entity, self.to),
        ]
    }
}

pub struct RotateActionPlugin;

impl Plugin for RotateActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            rotate_action_system
                .label(EffectProducer)
                .label(ActionInterpreter)
                .after(ActionProducer),
        );
    }
}

fn rotate_action_system(
    mut actors: Query<&Actor>,
    mut events: EventReader<ActionEvent>,
    mut effects: EventWriter<EffectEvent>,
) {
    for action in events.iter().filter_map(|e| e.as_action::<RotateAction>()) {
        if let Ok(actor) = actors.get(action.entity()) {
            for effect in action.effects() {
                effects.send(effect);
            }
        }
    }
}
