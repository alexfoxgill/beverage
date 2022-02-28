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
pub struct StepAction(Entity);

impl StepAction {
    pub fn new(entity: Entity) -> StepAction {
        StepAction(entity)
    }
}
impl Action for StepAction {
    fn cost(&self) -> u8 {
        1
    }
}

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::new(StepAction(e))
}

pub fn handler(
    In(action): In<StepAction>,
    actor: Query<(&Actor, &HexPos, &Facing)>,
    occupied: Query<&HexPos, With<Actor>>,
    map_tiles: Query<(&HexPos, &MapTile)>,
) -> ActionResult {
    let entity = action.0;
    let cost = action.cost();
    let (actor, pos, facing) = actor
        .get(entity)
        .ok()
        .ok_or(AnyActionError::generic("Missing components"))?;

    let to = pos.get_facing(facing.0);
    if actor.actions_remaining < cost {
        return AnyActionError::res_generic("Insufficient actions");
    }
    if occupied.iter().any(|x| x.0 == to) {
        return AnyActionError::res_generic("Destination occupied");
    }

    if !map_tiles
        .iter()
        .any(|(x, tile)| x.0 == to && tile.terrain == Terrain::Floor)
    {
        return AnyActionError::res_generic("Destination not floor");
    }

    Ok(EffectQueue::new(EnergyCostEffect::new(entity, cost)).then(MoveEffect::new(entity, to)))
}
