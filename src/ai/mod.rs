use bevy::prelude::*;

use hex2d::*;
use itertools::*;
use rand::prelude::*;

use crate::animation::AnimatingState;
use crate::domain::actions::rotate::RotateAction;
use crate::domain::actions::step::StepAction;
use crate::domain::actions::strike::StrikeAction;
use crate::turn_engine::actions::ActionQueue;
use crate::turn_queue::TurnQueue;
use crate::{common::*, Player};

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AnimatingState::Still).with_system(generate_ai_actions),
        );
    }
}

#[derive(Component)]
pub enum AIBehaviour {
    Wandering,
    Chasing(Entity),
}

fn direction_diff(from: HexDirection, to: HexDirection) -> Angle {
    *Angle::all().iter().find(|&&a| from + a == to).unwrap()
}

fn field_of_view(pos: Coordinate, facing: HexDirection) -> impl Iterator<Item = Coordinate> {
    iterate(pos, move |&x| x + facing)
}

pub fn generate_ai_actions(
    mut ai: Query<(&HexPos, &Facing, &Actor, &mut AIBehaviour)>,
    targets: Query<(&HexPos, Entity), With<Player>>,
    turn_queue: Res<TurnQueue>,
    mut actions: ResMut<ActionQueue>,
) {
    if let Some(&entity) = turn_queue.head() {
        if let Ok((&HexPos(pos), &Facing(facing), actor, mut behaviour)) = ai.get_mut(entity) {
            if actor.actions_remaining > 0 {
                match *behaviour {
                    AIBehaviour::Wandering => {
                        let target = targets.iter().find_map(|(HexPos(target_pos), target)| {
                            if field_of_view(pos, facing)
                                .take(10)
                                .any(|x| &x == target_pos)
                            {
                                Some(target)
                            } else {
                                None
                            }
                        });

                        if let Some(target) = target {
                            println!("{entity:?} is now chasing {target:?}");
                            *behaviour = AIBehaviour::Chasing(target)
                        } else {
                            let rotation =
                                Angle::from_int::<i32>(rand::thread_rng().gen_range(1..=6));

                            actions.push(RotateAction::new(entity, rotation));
                            actions.push(StepAction::new(entity));
                        }
                    }
                    AIBehaviour::Chasing(e) => {
                        if let Ok((&HexPos(target_pos), _)) = targets.get(e) {
                            if let Some(dir) = pos.direction_to_cw(target_pos) {
                                let turn = direction_diff(facing, dir);
                                match turn {
                                    Forward => {
                                        if pos.distance(target_pos) == 1 {
                                            actions.push(StrikeAction::new(entity))
                                        } else {
                                            actions.push(StepAction::new(entity))
                                        }
                                    }
                                    x => actions.push(RotateAction::new(entity, x)),
                                }
                            } else {
                                *behaviour = AIBehaviour::Wandering
                            }
                        } else {
                            *behaviour = AIBehaviour::Wandering
                        }
                    }
                }
            }
        }
    }
}
