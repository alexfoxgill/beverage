use super::*;
use crate::effects::{
    energy_cost_effect::{ActionCost, EnergyCostEffect},
    move_effect::MoveEffect,
    EffectProducer,
};
use bevy::prelude::*;
use hex2d::Coordinate;

#[derive(Debug)]
pub struct MoveAction {
    entity: Entity,
    to: Coordinate,
}

impl MoveAction {
    pub fn event(entity: Entity, to: Coordinate) -> ActionEvent {
        ActionEvent(Box::new(MoveAction { entity, to }))
    }
}
impl Action for MoveAction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn entity(&self) -> Entity {
        self.entity
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![
            EnergyCostEffect::event(self.entity, ActionCost::Fixed(1)),
            MoveEffect::event(self.entity, self.to),
        ]
    }
}

pub struct MoveActionPlugin;

impl Plugin for MoveActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            move_action_system
                .label(EffectProducer)
                .label(ActionInterpreter)
                .after(ActionProducer),
        );
    }
}

fn move_action_system(
    mut actors: Query<&Actor>,
    mut events: EventReader<ActionEvent>,
    mut effects: EventWriter<EffectEvent>,
) {
    for action in events.iter().filter_map(|e| e.as_action::<MoveAction>()) {
        if let Ok(actor) = actors.get(action.entity()) {
            if actor.actions_remaining >= 1 {
                for effect in action.effects() {
                    effects.send(effect);
                }
            }
        }
    }
}
