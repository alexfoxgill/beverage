use crate::{
    common::{Actor, Facing, HexPos},
    domain::effects::{
        energy_cost::{ActionCost, EnergyCostEffect},
        kill::KillEffect,
    },
    turn_engine::{actions::Action, effects::EffectQueue},
};
use bevy::prelude::*;

#[derive(Debug)]
pub struct StrikeAction(Entity);

impl StrikeAction {
    pub fn new(attacker: Entity) -> StrikeAction {
        StrikeAction(attacker)
    }
}

impl Action for StrikeAction {}

pub fn handler(
    In(StrikeAction(attacker)): In<StrikeAction>,
    query: Query<(&HexPos, &Facing, &Actor)>,
    targets: Query<(&HexPos, Entity), With<Actor>>,
    mut effect_queue: ResMut<EffectQueue>,
) {
    if let Ok((pos, facing, actor)) = query.get(attacker) {
        if actor.actions_remaining > 0 {
            let coord_to_attack = pos.get_facing(facing.0);
            effect_queue.push(EnergyCostEffect::new(attacker, ActionCost::All));
            for (pos, e) in targets.iter() {
                if pos.0 == coord_to_attack {
                    effect_queue.push(KillEffect::new(e));
                }
            }
        }
    }
}
