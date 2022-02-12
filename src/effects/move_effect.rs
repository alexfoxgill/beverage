use bevy::prelude::*;
use hex2d::Coordinate;

use crate::common::HexPos;

use super::{Effect, EffectEvent, EffectOutcome, EffectProducer};

#[derive(Debug)]
pub struct MoveEffect {
    entity: Entity,
    to: Coordinate,
}

impl MoveEffect {
    pub fn event(entity: Entity, to: Coordinate) -> EffectEvent {
        EffectEvent(Box::new(MoveEffect { entity, to }))
    }
}

impl Effect for MoveEffect {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub struct MoveEffectPlugin;

impl Plugin for MoveEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            move_effect_system
                .label(EffectOutcome)
                .after(EffectProducer),
        );
    }
}

fn move_effect_system(
    mut positions: Query<&mut HexPos>,
    mut event_reader: EventReader<EffectEvent>,
) {
    for effect in event_reader
        .iter()
        .filter_map(|e| e.as_effect::<MoveEffect>())
    {
        if let Ok(mut pos) = positions.get_mut(effect.entity) {
            pos.0 = effect.to;
        }
    }
}
