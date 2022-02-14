use bevy::prelude::*;

use crate::turn_engine::{effects::Effect, TurnSchedules};

#[derive(Debug)]
pub struct KillEffect {
    entity: Entity,
}

impl KillEffect {
    pub fn new(entity: Entity) -> KillEffect {
        KillEffect { entity }
    }
}

impl Effect for KillEffect {}

pub struct KillEffectPlugin;

impl Plugin for KillEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut schedules: ResMut<TurnSchedules>) {
    schedules.register_effect_handler(handler.system());
}

fn handler(effect: In<KillEffect>, mut commands: Commands) {
    commands.entity(effect.0.entity).despawn_recursive();
}
