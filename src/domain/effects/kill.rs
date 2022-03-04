use bevy::prelude::*;

use crate::turn_engine::effects::Effect;

#[derive(Debug, Clone)]
pub struct KillEffect(Entity);

impl KillEffect {
    pub fn new(entity: Entity) -> KillEffect {
        KillEffect(entity)
    }
}

impl Effect for KillEffect {}

pub fn handler(In(KillEffect(entity)): In<KillEffect>, mut commands: Commands) {
    commands.entity(entity).despawn_recursive();
}
