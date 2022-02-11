use bevy::prelude::*;

use hex2d::*;
use rand::prelude::*;

use crate::action_event::*;
use crate::common::*;
use crate::turn_queue::*;

pub struct IntentionEvent(Entity, Intention);

pub enum Intention {
    Rotate(Angle),
    Move(Angle),
    Attack(Angle),
    EndTurn,
}

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

pub fn generate_ai_intentions(
    actors: Query<(&HexPos, &Facing, &Actor)>,
    turn_queue: Res<TurnQueue>,
    mut ev_intention: EventWriter<IntentionEvent>,
) {
    if let Some(&entity) = turn_queue.queue.front() {
        if let Ok((_, _, actor)) = actors.get(entity) {
            if actor.control_source == ControlSource::AI {
                let rotation = Angle::from_int(rand::thread_rng().gen_range(1..=6));

                ev_intention.send(IntentionEvent(entity, Intention::Rotate(rotation)));
                ev_intention.send(IntentionEvent(entity, Intention::Move(Angle::Forward)));
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
