use bevy::prelude::*;

use crate::common::{Facing, HexDirection};

use crate::turn_engine::effects::Effect;
use crate::turn_engine::TurnSchedules;

#[derive(Debug)]
pub struct FaceEffect {
    entity: Entity,
    face: HexDirection,
}

impl FaceEffect {
    pub fn new(entity: Entity, face: HexDirection) -> FaceEffect {
        FaceEffect { entity, face }
    }
}

impl Effect for FaceEffect {}

pub struct FaceEffectPlugin;

impl Plugin for FaceEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    schedules.register_effect_handler(handler.system());
}

fn handler(effect: In<FaceEffect>, mut facings: Query<&mut Facing>) {
    if let Ok(mut facing) = facings.get_mut(effect.0.entity) {
        facing.0 = effect.0.face;
    }
}
