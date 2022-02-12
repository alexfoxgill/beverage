use super::*;
use crate::effects::{
    energy_cost_effect::{ActionCost, EnergyCostEffect},
    EffectProducer,
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct EndTurnAction {
    entity: Entity,
}

impl EndTurnAction {
    pub fn event(entity: Entity) -> ActionEvent {
        ActionEvent(Box::new(EndTurnAction { entity }))
    }
}

impl Action for EndTurnAction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn entity(&self) -> Entity {
        self.entity
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![EnergyCostEffect::event(self.entity, ActionCost::All)]
    }
}

pub struct EndTurnActionPlugin;

impl Plugin for EndTurnActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            end_turn_action_system
                .label(EffectProducer)
                .label(ActionInterpreter)
                .after(ActionProducer),
        );
    }
}

fn end_turn_action_system(
    mut actors: Query<&Actor>,
    mut events: EventReader<ActionEvent>,
    mut effects: EventWriter<EffectEvent>,
) {
    for action in events.iter().filter_map(|e| e.as_action::<EndTurnAction>()) {
        if let Ok(actor) = actors.get(action.entity()) {
            for effect in action.effects() {
                effects.send(effect);
            }
        }}
}
