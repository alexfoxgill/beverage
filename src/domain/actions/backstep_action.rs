use crate::{
    common::*,
    domain::effects::{
        energy_cost_effect::{ActionCost, EnergyCostEffect},
        move_effect::MoveEffect,
    },
    map::MapTile,
    turn_engine::{
        action_queue::{ActionSchedules, CurrentAction},
        actions::{Action, ActionEvent},
        effects::EffectEvent,
    },
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct BackstepAction {
    entity: Entity,
}

impl BackstepAction {
    pub fn event(entity: Entity) -> ActionEvent {
        ActionEvent(Box::new(BackstepAction { entity }))
    }
}
impl Action for BackstepAction {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn insert_resource(&self, world: &mut World) {
        let action = CurrentAction(self.clone());
        world.insert_resource(action)
    }
}

pub struct BackstepActionPlugin;

impl Plugin for BackstepActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<ActionSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_handler::<BackstepAction>(schedule)
}

fn handler(
    actors: Query<(&Actor, &HexPos, &Facing)>,
    map_tiles: Query<&HexPos, With<MapTile>>,
    action: Res<CurrentAction<BackstepAction>>,
    mut effects: EventWriter<EffectEvent>,
) {
    if let Ok((actor, pos, facing)) = actors.get(action.0.entity) {
        let cost = 2;
        if actor.actions_remaining >= cost {
            let to = pos.get_facing(-facing.0);

            if map_tiles.iter().any(|x| x.0 == to) {
                effects.send(EnergyCostEffect::event(
                    action.0.entity,
                    ActionCost::Fixed(cost),
                ));
                effects.send(MoveEffect::event(action.0.entity, to));
            }
        }
    }
}
