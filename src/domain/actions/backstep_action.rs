use crate::{
    common::*,
    domain::effects::{
        energy_cost_effect::{ActionCost, EnergyCostEffect},
        move_effect::MoveEffect,
    },
    map::MapTile,
    turn_engine::{actions::Action, effects::EffectQueue, TurnSystems},
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct BackstepAction(Entity);

impl BackstepAction {
    pub fn new(entity: Entity) -> BackstepAction {
        BackstepAction(entity)
    }
}
impl Action for BackstepAction {}

pub struct BackstepActionPlugin;

impl Plugin for BackstepActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_action_handler(handler.system())
}

fn handler(
    In(BackstepAction(entity)): In<BackstepAction>,
    actors: Query<(&Actor, &HexPos, &Facing)>,
    map_tiles: Query<&HexPos, With<MapTile>>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok((actor, pos, facing)) = actors.get(entity) {
        let cost = 2;
        if actor.actions_remaining >= cost {
            let to = pos.get_facing(-facing.0);

            if map_tiles.iter().any(|x| x.0 == to) {
                effect_queue.push(EnergyCostEffect::new(entity, ActionCost::Fixed(cost)));
                effect_queue.push(MoveEffect::new(entity, to));
            }
        }
    }
}
