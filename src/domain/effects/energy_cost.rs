use bevy::prelude::*;

use crate::domain::common::Actor;

use crate::turn_engine::effects::{Effect, EffectQueue};

use super::end_turn::EndTurnEffect;

#[derive(Debug)]
pub struct EnergyCostEffect(Entity, ActionCost);

#[derive(Debug, Clone)]
pub enum ActionCost {
    All,
    Fixed(u8),
    None,
}

impl EnergyCostEffect {
    pub fn new(entity: Entity, cost: ActionCost) -> EnergyCostEffect {
        EnergyCostEffect(entity, cost)
    }
}

impl Effect for EnergyCostEffect {}

pub fn handler(
    In(EnergyCostEffect(entity, cost)): In<EnergyCostEffect>,
    mut actors: Query<&mut Actor>,
    mut effects: ResMut<EffectQueue>,
) {
    if let Ok(mut actor) = actors.get_mut(entity) {
        match cost {
            ActionCost::All => {
                actor.actions_remaining = 0;
            }
            ActionCost::Fixed(cost) => {
                actor.actions_remaining = if cost < actor.actions_remaining {
                    actor.actions_remaining - cost
                } else {
                    0
                };
            }
            ActionCost::None => (),
        }

        if actor.actions_remaining == 0 {
            effects.push(EndTurnEffect::new(entity));
        }
    }
}
