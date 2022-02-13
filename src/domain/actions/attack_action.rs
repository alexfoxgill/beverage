use crate::{
    domain::effects::{
        energy_cost_effect::{ActionCost, EnergyCostEffect},
        kill_effect::KillEffect,
    },
    turn_engine::{
        actions::{
            action_queue::{ActionSchedules, CurrentAction},
            Action, ActionEvent,
        },
        effects::EffectEvent,
    },
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
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

    fn insert_resource(&self, world: &mut World) {
        let action = CurrentAction(self.clone());
        world.insert_resource(action)
    }
}

pub struct AttackActionPlugin;

impl Plugin for AttackActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<ActionSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_handler::<AttackAction>(schedule)
}

fn handler(action: Res<CurrentAction<AttackAction>>, mut effects: EventWriter<EffectEvent>) {
    effects.send(EnergyCostEffect::event(action.0.attacker, ActionCost::All));
    effects.send(KillEffect::event(action.0.victim));
}
