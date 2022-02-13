use std::time::Duration;

use crate::common::*;
use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType};

#[derive(Component)]
pub struct Animating(Timer);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AnimatingState {
    Animating,
    Still,
}

pub fn animate_movement(
    mut commands: Commands,
    query: Query<(&Transform, &HexPos, &Facing, Entity), Or<(Changed<HexPos>, Changed<Facing>)>>,
) {
    for (transform, pos, facing, e) in query.iter() {
        let new_translation = pos.as_translation(HEX_SPACING);
        let new_rotation = facing.as_rotation();

        let new_transform = transform
            .with_translation(new_translation)
            .with_rotation(new_rotation);

        let duration = Duration::from_millis(100);

        commands
            .entity(e)
            .insert(transform.ease_to(
                new_transform,
                EaseFunction::QuarticIn,
                EasingType::Once { duration },
            ))
            .insert(Animating(Timer::new(duration, false)));
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
