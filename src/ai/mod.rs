pub mod wandering;
use bevy::prelude::*;

use hex2d::*;
use rand::prelude::*;

use crate::actions::move_action::MoveAction;
use crate::actions::rotate_action::RotateAction;
use crate::actions::ActionEvent;
use crate::actions::ActionProducer;
use crate::animation::AnimatingState;
use crate::common::*;
use crate::turn_queue::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AnimatingState::Still)
                .with_system(generate_ai_actions)
                .label(ActionProducer),
        );
    }
}

pub fn generate_ai_actions(
    actors: Query<(&HexPos, &Facing, &Actor)>,
    turn_queue: Res<TurnQueue>,
    mut actions: EventWriter<ActionEvent>,
) {
    if let Some(&entity) = turn_queue.queue.front() {
        if let Ok((pos, facing, actor)) = actors.get(entity) {
            if actor.control_source == ControlSource::AI {
                let rotation = Angle::from_int(rand::thread_rng().gen_range(1..=6));

                let target = facing.rotated(rotation);
                actions.send(RotateAction::event(entity, target));
                actions.send(MoveAction::event(entity, pos.get_facing(target)));
            }
        }
    }
}
