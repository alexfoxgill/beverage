use crate::{
    domain::common::*,
    domain::effects::{energy_cost::EnergyCostEffect, move_entity::MoveEffect},
    map::{MapTile, Terrain},
    turn_engine::{
        actions::{Action, ActionQueue},
        effects::EffectQueue,
    },
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct BackstepAction(Entity);

impl BackstepAction {
    pub fn new(entity: Entity) -> BackstepAction {
        BackstepAction(entity)
    }
}
impl Action for BackstepAction {
    fn cost(&self) -> u8 {
        2
    }
}

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::new(BackstepAction(e))
}

pub fn handler(
    In(action): In<BackstepAction>,
    actor: Query<(&Actor, &HexPos, &Facing)>,
    occupied: Query<&HexPos, With<Actor>>,
    map_tiles: Query<(&HexPos, &MapTile)>,
) -> EffectQueue {
    let entity = action.0;
    let cost = action.cost();
    if let Ok((actor, pos, facing)) = actor.get(entity) {
        let to = pos.get_facing(-facing.0);
        if actor.actions_remaining < cost {
            return Default::default();
        }
        if occupied.iter().any(|x| x.0 == to) {
            return Default::default();
        }
        if map_tiles
            .iter()
            .any(|(x, tile)| x.0 == to && tile.terrain == Terrain::Floor)
        {
            return EffectQueue::new(EnergyCostEffect::new(entity, cost))
                .with(MoveEffect::new(entity, to));
        }
    }
    Default::default()
}
