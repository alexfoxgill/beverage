use bevy::prelude::*;

use crate::turn_engine::{
    effects::{Effect, EffectEvent},
    EffectDispatcher, Handled,
};

#[derive(Debug, Clone)]
pub struct KillEffect {
    entity: Entity,
}

impl KillEffect {
    pub fn event(entity: Entity) -> EffectEvent {
        EffectEvent(Box::new(KillEffect { entity }))
    }
}

impl Effect for KillEffect {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct KillEffectPlugin;

impl Plugin for KillEffectPlugin {
    fn build(&self, app: &mut App) {
        app.stage(EffectDispatcher, |stage: &mut SystemStage| {
            stage.add_system(kill_effect_system)
        });
    }
}

fn kill_effect_system(mut commands: Commands, mut event_reader: EventReader<EffectEvent>) {
    for effect in event_reader
        .iter()
        .filter_map(|e| e.as_effect::<KillEffect>())
    {
        commands.entity(effect.entity).despawn_recursive();
    }
}
