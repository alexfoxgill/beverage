use bevy::prelude::*;

use hex2d::*;
use rand::prelude::*;

use crate::domain::actions::end_turn::EndTurnAction;
use crate::domain::actions::rotate::RotateAction;
use crate::domain::actions::step::StepAction;
use crate::domain::actions::strike::StrikeAction;
use crate::domain::common::*;
use crate::domain::turn_queue::TurnQueue;
use crate::domain::vision::Vision;
use crate::map::MapTiles;
use crate::pathfinding::{a_star, Move};
use crate::turn_engine::actions::ActionQueue;
use crate::turn_engine::TurnState;
use crate::Player;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(generate_ai_actions);
    }
}

#[derive(Component)]
pub enum AIBehaviour {
    Wandering,
    Chasing(Entity),
}

pub fn generate_ai_actions(
    turn_state: Res<TurnState>,
    mut ai: Query<(&HexPos, &Facing, &Actor, &Vision, &mut AIBehaviour)>,
    targets: Query<(&HexPos, Entity), With<Player>>,
    map: MapTiles,
    turn_queue: Res<TurnQueue>,
    mut actions: ResMut<ActionQueue>,
) {
    if !actions.is_empty() {
        return;
    }
    if let &TurnState::Idle = turn_state.as_ref() {
        if let Some(&entity) = turn_queue.head() {
            if let Ok((&HexPos(pos), &Facing(facing), actor, vision, mut behaviour)) =
                ai.get_mut(entity)
            {
                if actor.actions_remaining == 0 {
                    return;
                }

                let walls = map.get_walls();
                let position = hex2d::Position::new(pos, facing);
                match *behaviour {
                    AIBehaviour::Wandering => {
                        let target = targets.iter().find_map(|(HexPos(target_pos), target)| {
                            if vision
                                .can_see_relative(position, *target_pos, |x| walls.contains(&x))
                            {
                                Some(target)
                            } else {
                                None
                            }
                        });
                        if let Some(target) = target {
                            *behaviour = AIBehaviour::Chasing(target)
                        } else {
                            let rotation =
                                Angle::from_int::<i32>(rand::thread_rng().gen_range(1..=6));

                            actions.push(RotateAction::new(entity, rotation));
                            actions.push(StepAction::new(entity));
                            actions.push(EndTurnAction::new(entity));
                        }
                    }
                    AIBehaviour::Chasing(target) => {
                        if let Ok((&HexPos(target_pos), _)) = targets.get(target) {
                            let valid_tiles = map.get_floor();

                            if let Some(mut path) =
                                a_star(position, target_pos, |x| valid_tiles.contains(x))
                            {
                                let mut cost = 0;
                                while cost < actor.actions_remaining {
                                    if let Some(next) = path.pop_front() {
                                        match next {
                                            Move::TurnLeft => {
                                                cost += actions
                                                    .push(RotateAction::new(entity, Angle::Left));
                                            }
                                            Move::TurnRight => {
                                                cost += actions
                                                    .push(RotateAction::new(entity, Angle::Right));
                                            }
                                            Move::StepForward => {
                                                // if this is the last move then we are adjacent to the target
                                                if path.is_empty() {
                                                    cost += actions.push(StrikeAction::new(entity));
                                                } else {
                                                    cost += actions.push(StepAction::new(entity));
                                                }
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                actions.push(EndTurnAction::new(entity));
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
