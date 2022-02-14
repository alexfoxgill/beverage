use crate::{
    domain::effects::{
        energy_cost_effect::{ActionCost, EnergyCostEffect},
        kill_effect::KillEffect,
    },
    turn_engine::{actions::Action, effects::EffectQueue, TurnSchedules},
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

fn setup(mut schedules: ResMut<TurnSchedules>) {
    schedules.register_action_handler(handler.system())
}

fn handler(action: In<AttackAction>, mut effect_queue: ResMut<EffectQueue>) {
    effect_queue.push(EnergyCostEffect::new(action.0.attacker, ActionCost::All));
    effect_queue.push(KillEffect::new(action.0.victim));
}
