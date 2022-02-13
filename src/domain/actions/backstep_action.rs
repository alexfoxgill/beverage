use crate::{
    common::*,
    domain::effects::{
        energy_cost_effect::{ActionCost, EnergyCostEffect},
        move_effect::MoveEffect,
    },
    map::MapTile,
    turn_engine::{
        actions::{Action, ActionEvent},
        effects::{EffectQueue},
        Handled, TurnSchedules,
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
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct BackstepActionPlugin;

impl Plugin for BackstepActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_action_handler::<BackstepAction>(schedule)
}

fn handler(
    actors: Query<(&Actor, &HexPos, &Facing)>,
    map_tiles: Query<&HexPos, With<MapTile>>,
    action: Res<Handled<BackstepAction>>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok((actor, pos, facing)) = actors.get(action.0.entity) {
        let cost = 2;
        if actor.actions_remaining >= cost {
            let to = pos.get_facing(-facing.0);

            if map_tiles.iter().any(|x| x.0 == to) {
                effect_queue.push(EnergyCostEffect::event(
                    action.0.entity,
                    ActionCost::Fixed(cost),
                ));
                effect_queue.push(MoveEffect::event(action.0.entity, to));
            }
        }
    }
}
