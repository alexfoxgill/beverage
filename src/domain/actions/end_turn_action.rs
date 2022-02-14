use crate::{
    domain::effects::energy_cost_effect::{ActionCost, EnergyCostEffect},
    turn_engine::{actions::Action, effects::EffectQueue, Handled, TurnSchedules},
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct EndTurnAction {
    entity: Entity,
}

impl EndTurnAction {
    pub fn new(entity: Entity) -> EndTurnAction {
        EndTurnAction { entity }
    }
}

impl Action for EndTurnAction {
    fn insert_handled(self: Box<Self>, world: &mut World) {
        world.insert_resource(Handled(*self));
    }
}

pub struct EndTurnActionPlugin;

impl Plugin for EndTurnActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    schedules.register_action_system(handler.system())
}

fn handler(In(action): In<EndTurnAction>, mut effects: ResMut<EffectQueue>) {
    effects.push(EnergyCostEffect::new(action.entity, ActionCost::All));
}
