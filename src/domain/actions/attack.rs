use crate::{
    domain::effects::{
        energy_cost::{ActionCost, EnergyCostEffect},
        kill::KillEffect,
    },
    turn_engine::{actions::Action, effects::EffectQueue, TurnSystems},
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct AttackAction {
    attacker: Entity,
    victim: Entity,
}

impl AttackAction {
    pub fn new(attacker: Entity, victim: Entity) -> AttackAction {
        AttackAction { attacker, victim }
    }
}

impl Action for AttackAction {}

pub struct AttackActionPlugin;

impl Plugin for AttackActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_action_handler(handler.system())
}

fn handler(
    In(AttackAction { attacker, victim }): In<AttackAction>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    effect_queue.push(EnergyCostEffect::new(attacker, ActionCost::All));
    effect_queue.push(KillEffect::new(victim));
}
