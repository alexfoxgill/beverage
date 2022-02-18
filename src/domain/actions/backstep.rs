use crate::{
    domain::common::*,
    domain::effects::{
        energy_cost::{ActionCost, EnergyCostEffect},
        move_entity::MoveEffect,
    },
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
impl Action for BackstepAction {}

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::new(BackstepAction(e))
}

pub fn handler(
    In(BackstepAction(entity)): In<BackstepAction>,
    actor: Query<(&Actor, &HexPos, &Facing)>,
    occupied: Query<&HexPos, With<Actor>>,
    map_tiles: Query<(&HexPos, &MapTile)>,
) -> EffectQueue {
    if let Ok((actor, pos, facing)) = actor.get(entity) {
        let cost = 2;
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
            return EffectQueue::new(EnergyCostEffect::new(entity, ActionCost::Fixed(cost)))
                .with(MoveEffect::new(entity, to));
        }
    }
    Default::default()
}
