use bevy::prelude::*;

use crate::turn_engine::{effects::Effect, TurnSystems};

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

fn setup(mut systems: ResMut<TurnSystems>) {
    systems.register_effect_handler(handler.system());
}

fn handler(In(effect): In<KillEffect>, mut commands: Commands) {
    commands.entity(effect.entity).despawn_recursive();
}
