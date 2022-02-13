use bevy::prelude::*;

use crate::common::Actor;

use crate::turn_engine::effects::Effect;
use crate::turn_engine::{Handled, TurnSchedules};

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

impl Effect for EnergyCostEffect {
    fn insert_handled(self: Box<Self>, world: &mut World) {
        world.insert_resource(Handled(*self));
    }
}

pub struct EnergyCostEffectPlugin;

impl Plugin for EnergyCostEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    let mut schedule = Schedule::default();
    schedule.add_stage("only", SystemStage::single_threaded().with_system(handler));
    schedules.register_effect_handler::<EnergyCostEffect>(schedule)
}

fn handler(mut actors: Query<&mut Actor>, effect: Res<Handled<EnergyCostEffect>>) {
    match effect.0.cost {
        ActionCost::All => {
            if let Ok(mut actor) = actors.get_mut(effect.0.entity) {
                actor.actions_remaining = 0;
            }
        }
        ActionCost::Fixed(cost) => {
            if let Ok(mut actor) = actors.get_mut(effect.0.entity) {
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
