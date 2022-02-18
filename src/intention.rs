use bevy::prelude::*;

use hex2d::*;

use crate::domain::actions::backstep::BackstepAction;
use crate::domain::actions::end_turn::EndTurnAction;
use crate::domain::actions::rotate::RotateAction;
use crate::domain::actions::step::StepAction;
use crate::domain::actions::strike::StrikeAction;
use crate::domain::turn_queue::*;
use crate::turn_engine::actions::ActionQueue;

#[derive(Component)]
pub struct PlayerControlled;

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
    TurnLeft,
    TurnRight,
    Strike,
    EndTurn,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
struct IntentionProducer;

fn ingame_keyboard_input(
    keys: Res<Input<KeyCode>>,
    players: Query<(), With<PlayerControlled>>,
    turn_queue: Res<TurnQueue>,
    mut ev_intention: EventWriter<IntentionEvent>,
) {
    if let Some(&entity) = turn_queue.head() {
        if players.get(entity).is_ok() {
            let intention = if keys.just_pressed(KeyCode::Left) {
                Intention::TurnLeft
            } else if keys.just_pressed(KeyCode::Right) {
                Intention::TurnRight
            } else if keys.just_pressed(KeyCode::Up) {
                Intention::Step
            } else if keys.just_pressed(KeyCode::Down) {
                Intention::Backstep
            } else if keys.just_pressed(KeyCode::E) {
                Intention::EndTurn
            } else if keys.just_pressed(KeyCode::Space) {
                Intention::Strike
            } else {
                return;
            };
            ev_intention.send(IntentionEvent(entity, intention));
        }
    }
}

fn process_intention(
    mut ev_intention: EventReader<IntentionEvent>,
    mut ev_action: ResMut<ActionQueue>,
) {
    for IntentionEvent(entity, intention) in ev_intention.iter() {
        match intention {
            Intention::TurnLeft => ev_action.push(RotateAction::new(*entity, Angle::Left)),
            Intention::TurnRight => ev_action.push(RotateAction::new(*entity, Angle::Right)),
            Intention::Step => ev_action.push(StepAction::new(*entity)),
            Intention::Backstep => ev_action.push(BackstepAction::new(*entity)),
            Intention::EndTurn => ev_action.push(EndTurnAction::new(*entity)),
            Intention::Strike => ev_action.push(StrikeAction::new(*entity)),
        }
    }
}
