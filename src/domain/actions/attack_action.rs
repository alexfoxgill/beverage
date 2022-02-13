use crate::{
    domain::effects::{
        energy_cost_effect::{ActionCost, EnergyCostEffect},
        kill_effect::KillEffect,
    },
    turn_engine::{actions::Action, effects::EffectQueue, Handled, TurnSchedules},
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct AttackAction {
    attacker: Entity,
    victim: Entity,
}

impl AttackAction {
    pub fn new(attacker: Entity, victim: Entity) -> AttackAction {
        AttackAction { attacker, victim }
    }
}

impl Action for AttackAction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct AttackActionPlugin;

impl Plugin for AttackActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_action_handler::<AttackAction>(schedule)
}

fn handler(action: Res<Handled<AttackAction>>, mut effect_queue: ResMut<EffectQueue>) {
    effect_queue.push(EnergyCostEffect::event(action.0.attacker, ActionCost::All));
    effect_queue.push(KillEffect::event(action.0.victim));
}
