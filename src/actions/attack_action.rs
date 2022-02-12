use super::*;
use crate::effects::{
    energy_cost_effect::{ActionCost, EnergyCostEffect},
    kill_effect::KillEffect,
    EffectProducer,
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct AttackAction {
    attacker: Entity,
    victim: Entity,
}

impl AttackAction {
    pub fn event(attacker: Entity, victim: Entity) -> ActionEvent {
        ActionEvent(Box::new(AttackAction { attacker, victim }))
    }
}

impl Action for AttackAction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn entity(&self) -> Entity {
        self.attacker
    }

    fn effects(&self) -> Vec<EffectEvent> {
        vec![
            EnergyCostEffect::event(self.attacker, ActionCost::All),
            KillEffect::event(self.victim),
        ]
    }
}

pub struct AttackActionPlugin;

impl Plugin for AttackActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            attack_action_system
                .label(EffectProducer)
                .label(ActionInterpreter)
                .after(ActionProducer),
        );
    }
}

fn attack_action_system(
    actors: Query<&Actor>,
    mut events: EventReader<ActionEvent>,
    mut effects: EventWriter<EffectEvent>,
) {
    for action in events.iter().filter_map(|e| e.as_action::<AttackAction>()) {
        if let Ok(actor) = actors.get(action.entity()) {
            for effect in action.effects() {
                effects.send(effect);
            }
        }
    }
}
