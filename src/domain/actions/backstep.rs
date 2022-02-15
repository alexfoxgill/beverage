use crate::{
    common::*,
    domain::effects::{
        energy_cost::{ActionCost, EnergyCostEffect},
        move_entity::MoveEffect,
    },
    map::MapTile,
    turn_engine::{actions::Action, effects::EffectQueue},
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

pub fn handler(
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
