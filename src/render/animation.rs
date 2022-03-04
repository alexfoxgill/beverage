use std::time::Duration;

use crate::{
    domain::{
        common::*,
        effects::{face::FaceEffect, move_entity::MoveEffect},
    },
    turn_engine::{TurnStage, TurnState},
};
use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AnimatingState::Still).add_stage_after(
            TurnStage::Action,
            "ActionAnimationStage",
            SystemStage::parallel()
                .with_system(initiate_animation)
                .with_system(update_animating_state),
        );
    }
}

#[derive(Component)]
pub struct Animating(Timer);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AnimatingState {
    Animating,
    Still,
}

pub fn initiate_animation(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    transforms: Query<&Transform>,
) {
    if let TurnState::Executing { effects, .. } = turn_state.as_ref() {
        if let Some(MoveEffect(e, to)) = effects.find() {
            if let Ok(transform) = transforms.get(*e) {
                let duration = Duration::from_millis(150);
                let new_translation = HexPos(*to).as_translation(HEX_SPACING);
                let new_transform = transform.with_translation(new_translation);

                commands
                    .entity(*e)
                    .insert(transform.ease_to(
                        new_transform,
                        EaseFunction::QuadraticInOut,
                        EasingType::Once { duration },
                    ))
                    .insert(Animating(Timer::new(duration, false)));
            }
        }

        if let Some(FaceEffect(e, dir)) = effects.find() {
            if let Ok(transform) = transforms.get(*e) {
                let duration = Duration::from_millis(150);
                let new_rotation = Facing(*dir).as_rotation();
                let new_transform = transform.with_rotation(new_rotation);

                commands
                    .entity(*e)
                    .insert(transform.ease_to(
                        new_transform,
                        EaseFunction::QuadraticInOut,
                        EasingType::Once { duration },
                    ))
                    .insert(Animating(Timer::new(duration, false)));
            }
        }
    }
}

pub fn update_animating_state(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<State<AnimatingState>>,
    mut query: Query<(&mut Animating, Entity)>,
) {
    let current_state = state.current();
    let mut is_animating = false;
    for (mut anim, e) in query.iter_mut() {
        anim.0.tick(time.delta());

        if anim.0.finished() {
            commands.entity(e).remove::<Animating>();
        } else {
            is_animating = true;
        }
    }

    match (is_animating, current_state) {
        (true, AnimatingState::Still) => {
            state.set(AnimatingState::Animating).unwrap();
        }
        (false, AnimatingState::Animating) => {
            state.set(AnimatingState::Still).unwrap();
        }
        _ => (),
    }
}
