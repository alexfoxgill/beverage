pub mod wandering;
use bevy::prelude::*;

use hex2d::*;
use rand::prelude::*;

use crate::animation::AnimatingState;
use crate::common::*;
use crate::domain::actions::rotate::RotateAction;
use crate::domain::actions::step::StepAction;
use crate::turn_engine::actions::ActionQueue;
use crate::turn_queue::TurnQueue;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AnimatingState::Still).with_system(generate_ai_actions),
        );
    }
}

pub fn generate_ai_actions(
    actors: Query<(&HexPos, &Facing, &Actor)>,
    turn_queue: Res<TurnQueue>,
    mut actions: ResMut<ActionQueue>,
) {
    if let Some(&entity) = turn_queue.head() {
        if let Ok((_pos, _facing, actor)) = actors.get(entity) {
            if actor.control_source == ControlSource::AI && actor.actions_remaining > 0 {
                let rotation = Angle::from_int(rand::thread_rng().gen_range(1..=6));

                actions.push(RotateAction::new(entity, rotation));
                actions.push(StepAction::new(entity));
            }
        }
    }
}
