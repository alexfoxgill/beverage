use bevy::prelude::*;

use crate::common::Actor;

use crate::turn_engine::Handled;
use crate::turn_engine::{
    effects::{Effect, EffectEvent},
    EffectDispatcher,
};

#[derive(Debug, Clone)]
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
    pub fn event(entity: Entity, cost: ActionCost) -> EffectEvent {
        EffectEvent(Box::new(EnergyCostEffect { entity, cost }))
    }
}

impl Effect for EnergyCostEffect {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct EnergyCostEffectPlugin;

impl Plugin for EnergyCostEffectPlugin {
    fn build(&self, app: &mut App) {
        app.stage(EffectDispatcher, |stage: &mut SystemStage| {
            stage.add_system(energy_cost_effect_system)
        });
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
        println!("Enacting energy cost for {:?}", effect.entity);
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
