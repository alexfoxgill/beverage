use bevy::prelude::*;

use crate::turn_engine::{effects::Effect, TurnSystems};

#[derive(Debug)]
pub struct KillEffect(Entity);

impl KillEffect {
    pub fn new(entity: Entity) -> KillEffect {
        KillEffect(entity)
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

fn handler(In(KillEffect(entity)): In<KillEffect>, mut commands: Commands) {
    commands.entity(entity).despawn_recursive();
}
