use crate::{
    domain::effects::{
        energy_cost::{ActionCost, EnergyCostEffect},
        kill::KillEffect,
    },
    turn_engine::{actions::Action, effects::EffectQueue},
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

pub fn handler(
    In(AttackAction { attacker, victim }): In<AttackAction>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    effect_queue.push(EnergyCostEffect::new(attacker, ActionCost::All));
    effect_queue.push(KillEffect::new(victim));
}
