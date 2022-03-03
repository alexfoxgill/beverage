use bevy::prelude::*;

use crate::domain::common::Actor;

use crate::turn_engine::effects::Effect;

#[derive(Debug)]
pub struct EnergyCostEffect(Entity, u8);

impl EnergyCostEffect {
    pub fn new(entity: Entity, cost: u8) -> EnergyCostEffect {
        EnergyCostEffect(entity, cost)
    }
}

impl Effect for EnergyCostEffect {}

pub fn handler(
    In(EnergyCostEffect(entity, cost)): In<EnergyCostEffect>,
    mut actors: Query<&mut Actor>,
) {
    if let Ok(mut actor) = actors.get_mut(entity) {
        if cost > actor.actions_remaining {
            actor.actions_remaining = 0;
        } else {
            actor.actions_remaining -= cost;
        }
    }
}
