use bevy::prelude::*;

use crate::common::{Facing, HexDirection};

use super::{Effect, EffectEvent, EffectOutcome, EffectProducer};

#[derive(Debug)]
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
}

pub struct FaceEffectPlugin;

impl Plugin for FaceEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            face_effect_system
                .label(EffectOutcome)
                .after(EffectProducer),
        );
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
