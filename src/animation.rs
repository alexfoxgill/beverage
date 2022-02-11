use std::time::Duration;

use crate::common::*;
use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType};

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

        commands.entity(e).insert(transform.ease_to(
            new_transform,
            EaseFunction::QuadraticInOut,
            EasingType::Once {
                duration: Duration::from_millis(50),
            },
        ));
    }
}
