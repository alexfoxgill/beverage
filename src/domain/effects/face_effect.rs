use bevy::prelude::*;

use crate::common::{Facing, HexDirection};

use crate::turn_engine::Handled;
use crate::turn_engine::{
    effects::{Effect, EffectEvent},
    EffectDispatcher,
};

#[derive(Debug, Clone)]
pub struct FaceEffect {
    entity: Entity,
    face: HexDirection,
}

impl FaceEffect {
    pub fn event(entity: Entity, face: HexDirection) -> EffectEvent {
        EffectEvent(Box::new(FaceEffect { entity, face }))
    }
}

impl Effect for FaceEffect {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn insert_resource(&self, world: &mut World) {
        world.insert_resource(Handled(self.clone()));
    }
}

pub struct FaceEffectPlugin;

impl Plugin for FaceEffectPlugin {
    fn build(&self, app: &mut App) {
        app.stage(EffectDispatcher, |stage: &mut SystemStage| {
            stage.add_system(face_effect_system)
        });
    }
}

fn face_effect_system(mut facings: Query<&mut Facing>, mut event_reader: EventReader<EffectEvent>) {
    for effect in event_reader
        .iter()
        .filter_map(|e| e.as_effect::<FaceEffect>())
    {
        if let Ok(mut facing) = facings.get_mut(effect.entity) {
            facing.0 = effect.face;
        }
    }
}
