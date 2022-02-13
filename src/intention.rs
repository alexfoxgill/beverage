use bevy::prelude::*;

use hex2d::*;

use crate::common::*;
use crate::domain::actions::attack_action::AttackAction;
use crate::domain::actions::backstep_action::BackstepAction;
use crate::domain::actions::end_turn_action::EndTurnAction;
use crate::domain::actions::rotate_action::RotateAction;
use crate::domain::actions::step_action::StepAction;
use crate::turn_engine::actions::ActionQueue;
use crate::turn_queue::*;

pub struct IntentionPlugin;

impl Plugin for IntentionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<IntentionEvent>()
            .add_system(ingame_keyboard_input.label(IntentionProducer))
            .add_system(process_intention.after(IntentionProducer));
    }
}

struct IntentionEvent(Entity, Intention);

#[derive(Debug)]
enum Intention {
    Step,
    Backstep,
    Rotate(Angle),
    Attack(Angle),
    EndTurn,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
struct IntentionProducer;

fn ingame_keyboard_input(
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
                    ev_intention.send(IntentionEvent(entity, Intention::Step));
                }
                if keys.just_pressed(KeyCode::Down) {
                    ev_intention.send(IntentionEvent(entity, Intention::Backstep));
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

fn process_intention(
    mut actors: Query<(&Facing, &HexPos, Entity)>,
    mut ev_intention: EventReader<IntentionEvent>,
    mut ev_action: ResMut<ActionQueue>,
) {
    for IntentionEvent(entity, intention) in ev_intention.iter() {
        let (facing, pos, _) = actors.get_mut(*entity).unwrap();

        match intention {
            Intention::Rotate(angle) => {
                let target = facing.rotated(*angle);
                ev_action.push(RotateAction::event(*entity, target));
            }
            Intention::Step => {
                ev_action.push(StepAction::event(*entity));
            }
            Intention::Backstep => {
                ev_action.push(BackstepAction::event(*entity));
            }
            Intention::EndTurn => {
                ev_action.push(EndTurnAction::event(*entity));
            }
            Intention::Attack(angle) => {
                let direction = facing.rotated(*angle);
                let coord_to_attack = pos.get_facing(direction);
                for (_, pos, e) in actors.iter() {
                    if pos.0 == coord_to_attack {
                        ev_action.push(AttackAction::event(*entity, e));
                    }
                }
            }
        }
    }
}
