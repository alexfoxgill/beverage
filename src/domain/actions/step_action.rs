use crate::{
    common::*,
    domain::effects::{
        energy_cost_effect::{ActionCost, EnergyCostEffect},
        move_effect::MoveEffect,
    },
    map::MapTile,
    turn_engine::{actions::Action, effects::EffectQueue, Handled, TurnSchedules},
};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct StepAction {
    entity: Entity,
}

impl StepAction {
    pub fn new(entity: Entity) -> StepAction {
        StepAction { entity }
    }
}
impl Action for StepAction {
    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct StepActionPlugin;

impl Plugin for StepActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let schedule =
        Schedule::default().with_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_action_handler::<StepAction>(schedule)
}

fn handler(
    actors: Query<(&Actor, &HexPos, &Facing)>,
    map_tiles: Query<&HexPos, With<MapTile>>,
    action: Res<Handled<StepAction>>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok((actor, pos, facing)) = actors.get(action.0.entity) {
        let cost = 1;
        if actor.actions_remaining >= cost {
            let to = pos.get_facing(facing.0);

            if map_tiles.iter().any(|x| x.0 == to) {
                effect_queue.push(EnergyCostEffect::new(
                    action.0.entity,
                    ActionCost::Fixed(cost),
                ));
                effect_queue.push(MoveEffect::new(action.0.entity, to));
            }
        }
    }
}
