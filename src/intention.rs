use bevy::prelude::*;

use hex2d::*;

use crate::actions::attack_action::AttackAction;
use crate::actions::end_turn_action::EndTurnAction;
use crate::actions::move_action::MoveAction;
use crate::actions::rotate_action::RotateAction;
use crate::actions::ActionEvent;
use crate::actions::ActionProducer;
use crate::common::*;
use crate::turn_queue::*;

pub struct IntentionPlugin;

impl Plugin for IntentionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ingame_keyboard_input.label(IntentionProducer))
            .add_system(
                process_intention
                    .label(ActionProducer)
                    .after(IntentionProducer),
            );
    }
}

pub struct IntentionEvent(Entity, Intention);

#[derive(Debug)]
pub enum Intention {
    Rotate(Angle),
    Move(Angle),
    Attack(Angle),
    EndTurn,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
pub struct IntentionProducer;

pub fn ingame_keyboard_input(
    keys: Res<Input<KeyCode>>,
    actors: Query<&Actor>,
    turn_queue: Res<TurnQueue>,
    mut ev_intention: EventWriter<IntentionEvent>,
) {
    if let Some(&entity) = turn_queue.queue.front() {
        if let Ok(actor) = actors.get(entity) {
            if actor.control_source == ControlSource::Player {
                if keys.just_pressed(KeyCode::Left) {
                    ev_intention.send(IntentionEvent(entity, Intention::Rotate(Angle::Left)));
                }
                if keys.just_pressed(KeyCode::Right) {
                    ev_intention.send(IntentionEvent(entity, Intention::Rotate(Angle::Right)));
                }
                if keys.just_pressed(KeyCode::Up) {
                    ev_intention.send(IntentionEvent(entity, Intention::Move(Angle::Forward)));
                }
                if keys.just_pressed(KeyCode::Down) {
                    ev_intention.send(IntentionEvent(entity, Intention::Move(Angle::Back)));
                }
                if keys.just_pressed(KeyCode::E) {
                    ev_intention.send(IntentionEvent(entity, Intention::EndTurn));
                }
                if keys.just_pressed(KeyCode::Space) {
                    ev_intention.send(IntentionEvent(entity, Intention::Attack(Angle::Forward)));
                }
            }
        }
    }
}

pub fn process_intention(
    mut actors: Query<(&Facing, &HexPos, Entity)>,
    mut ev_intention: EventReader<IntentionEvent>,
    mut ev_action: EventWriter<ActionEvent>,
) {
    for IntentionEvent(entity, intention) in ev_intention.iter() {
        let (facing, pos, _) = actors.get_mut(*entity).unwrap();

        match intention {
            Intention::Rotate(angle) => {
                let target = facing.rotated(*angle);
                ev_action.send(RotateAction::event(*entity, target));
            }
            Intention::Move(angle) => {
                let dir = facing.rotated(*angle);
                let target = pos.get_facing(dir);
                ev_action.send(MoveAction::event(*entity, target));
            }
            Intention::EndTurn => {
                ev_action.send(EndTurnAction::event(*entity));
            }
            Intention::Attack(angle) => {
                let direction = facing.rotated(*angle);
                let coord_to_attack = pos.get_facing(direction);
                for (_, pos, e) in actors.iter() {
                    if pos.0 == coord_to_attack {
                        ev_action.send(AttackAction::event(*entity, e));
                    }
                }
            }
        }
    }
}
