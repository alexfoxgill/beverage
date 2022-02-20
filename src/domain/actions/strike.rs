use crate::{
    domain::common::{Actor, Facing, HexPos},
    domain::effects::{energy_cost::EnergyCostEffect, kill::KillEffect},
    turn_engine::{
        actions::{Action, ActionQueue},
        effects::EffectQueue,
    },
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct StrikeAction(Entity);

impl StrikeAction {
    pub fn new(attacker: Entity) -> StrikeAction {
        StrikeAction(attacker)
    }
}

impl Action for StrikeAction {
    fn cost(&self) -> u8 {
        1
    }
}

pub fn generator(In(e): In<Entity>) -> ActionQueue {
    ActionQueue::new(StrikeAction(e))
}

pub fn handler(
    In(action): In<StrikeAction>,
    query: Query<(&HexPos, &Facing, &Actor)>,
    targets: Query<(&HexPos, Entity), With<Actor>>,
) -> EffectQueue {
    let attacker = action.0;
    let cost = action.cost();
    if let Ok((pos, facing, actor)) = query.get(attacker) {
        if actor.actions_remaining > 0 {
            let coord_to_attack = pos.get_facing(facing.0);
            let mut effects = EffectQueue::new(EnergyCostEffect::new(attacker, cost));

            for (pos, e) in targets.iter() {
                if pos.0 == coord_to_attack {
                    effects.push(KillEffect::new(e));
                }
            }
            return effects;
        }
    }
    Default::default()
}
