use bevy::prelude::*;

use crate::common::Actor;

use super::{Effect, EffectEvent, EffectOutcome, EffectProducer};

#[derive(Debug)]
pub struct EnergyCostEffect {
    pub entity: Entity,
    pub cost: ActionCost,
}

#[derive(Debug)]
pub enum ActionCost {
    All,
    Fixed(u8),
    None,
}

impl EnergyCostEffect {
    pub fn event(entity: Entity, cost: ActionCost) -> EffectEvent {
        EffectEvent(Box::new(EnergyCostEffect { entity, cost }))
    }
}

impl Effect for EnergyCostEffect {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct EnergyCostEffectPlugin;

impl Plugin for EnergyCostEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            energy_cost_effect_system
                .label(EffectOutcome)
                .after(EffectProducer),
        );
    }
}

fn energy_cost_effect_system(
    mut actors: Query<&mut Actor>,
    mut event_reader: EventReader<EffectEvent>,
) {
    for effect in event_reader
        .iter()
        .filter_map(|e| e.as_effect::<EnergyCostEffect>())
    {
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
}
