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
) -> EffectQueue {
    let mut effects = EffectQueue::default();
    if let Ok((actor, pos, facing)) = actors.get(entity) {
        let cost = 1;
        let to = pos.get_facing(facing.0);
        if actor.actions_remaining >= cost && map_tiles.iter().any(|x| x.0 == to) {
            effects.push(EnergyCostEffect::new(entity, ActionCost::Fixed(cost)));
            effects.push(MoveEffect::new(entity, to));
        }
    }
    effects
}
