use crate::{
    domain::common::*,
    domain::effects::{energy_cost::EnergyCostEffect, move_entity::MoveEffect},
    map::{MapTile, Terrain},
    turn_engine::{
        actions::{Action, ActionQueue, ActionResult, AnyActionError},
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
) -> ActionResult {
    let entity = action.0;
    let cost = action.cost();
    let (actor, pos, facing) = actor
        .get(entity)?;
    let to = pos.get_facing(-facing.0);
    if actor.actions_remaining < cost {
        return AnyActionError::res_generic("Not enough action points");
    }
    if occupied.iter().any(|x| x.0 == to) {
        return AnyActionError::res_generic("Destination occupied");
    }
    if !map_tiles
        .iter()
        .any(|(x, tile)| x.0 == to && tile.terrain == Terrain::Floor)
    {
        return AnyActionError::res_generic("Destination not floor tile");
    }

    Ok(EffectQueue::new(EnergyCostEffect::new(entity, cost)).then(MoveEffect::new(entity, to)))
}
