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
pub struct StepAction(Entity);

impl StepAction {
    pub fn new(entity: Entity) -> StepAction {
        StepAction(entity)
    }
}
impl Action for StepAction {}

pub fn handler(
    In(StepAction(entity)): In<StepAction>,
    actors: Query<(&Actor, &HexPos, &Facing)>,
    map_tiles: Query<&HexPos, With<MapTile>>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok((actor, pos, facing)) = actors.get(entity) {
        let cost = 1;
        if actor.actions_remaining >= cost {
            let to = pos.get_facing(facing.0);

            if map_tiles.iter().any(|x| x.0 == to) {
                effect_queue.push(EnergyCostEffect::new(entity, ActionCost::Fixed(cost)));
                effect_queue.push(MoveEffect::new(entity, to));
            }
        }
    }
}
