use crate::{
    domain::effects::energy_cost_effect::{ActionCost, EnergyCostEffect},
    turn_engine::{
        actions::{Action, ActionEvent},
        effects::EffectEvent,
        Handled, TurnSchedules,
    },
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
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

    fn insert_resource(&self, world: &mut World) {
        let action = Handled(self.clone());
        world.insert_resource(action)
    }
}

pub struct EndTurnActionPlugin;

impl Plugin for EndTurnActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_action_handler::<EndTurnAction>(schedule)
}

fn handler(action: Res<Handled<EndTurnAction>>, mut effects: EventWriter<EffectEvent>) {
    effects.send(EnergyCostEffect::event(action.0.entity, ActionCost::All));
}
