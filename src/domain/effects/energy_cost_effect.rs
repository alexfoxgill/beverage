use bevy::prelude::*;

use crate::common::Actor;

use crate::turn_engine::effects::Effect;
use crate::turn_engine::TurnSystems;

#[derive(Debug)]
pub struct EnergyCostEffect {
    pub entity: Entity,
    pub cost: ActionCost,
}

#[derive(Debug, Clone)]
pub enum ActionCost {
    All,
    Fixed(u8),
    None,
}

impl EnergyCostEffect {
    pub fn new(entity: Entity, cost: ActionCost) -> EnergyCostEffect {
        EnergyCostEffect { entity, cost }
    }
}

impl Effect for EnergyCostEffect {}

pub struct EnergyCostEffectPlugin;

impl Plugin for EnergyCostEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_effect_handler(handler.system());
}

fn handler(In(effect): In<EnergyCostEffect>, mut actors: Query<&mut Actor>) {
    match effect.cost {
        ActionCost::All => {
            if let Ok(mut actor) = actors.get_mut(effect.entity) {
                actor.actions_remaining = 0;
            }
        }
        ActionCost::Fixed(cost) => {
            if let Ok(mut actor) = actors.get_mut(effect.entity) {
                actor.actions_remaining = if cost < actor.actions_remaining {
                    actor.actions_remaining - cost
                } else {
                    0
                };
            }
        }
        ActionCost::None => (),
    }
}
