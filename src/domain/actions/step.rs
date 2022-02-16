use crate::{
    domain::common::*,
    domain::effects::{
        energy_cost::{ActionCost, EnergyCostEffect},
        move_entity::MoveEffect,
    },
    map::MapTile,
    turn_engine::{actions::{Action, ActionQueue}, effects::EffectQueue},
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

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::new(StepAction(e))
}

pub fn handler(
    In(StepAction(entity)): In<StepAction>,
    actor: Query<(&Actor, &HexPos, &Facing)>,
    occupied: Query<&HexPos, With<Actor>>,
    map_tiles: Query<&HexPos, With<MapTile>>,
) -> EffectQueue {
    if let Ok((actor, pos, facing)) = actor.get(entity) {
        let cost = 1;
        let to = pos.get_facing(facing.0);
        if actor.actions_remaining < cost {
            return Default::default();
        }
        if occupied.iter().any(|x| x.0 == to) {
            return Default::default();
        }
        if map_tiles.iter().any(|x| x.0 == to) {
            return EffectQueue::new(EnergyCostEffect::new(entity, ActionCost::Fixed(cost)))
                .with(MoveEffect::new(entity, to));
        }
    }
    Default::default()
}
