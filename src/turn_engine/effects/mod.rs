use std::any::Any;

use bevy::prelude::*;

use super::actions::ActionDispatcher;

pub mod effect_queue;

pub trait Effect: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct EffectEvent(pub Box<dyn Effect>);

impl EffectEvent {
    pub fn as_effect<T: Any + Effect>(&self) -> Option<&T> {
        self.0.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, StageLabel)]
pub struct EffectDispatcher;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EffectEvent>().add_stage_after(
            ActionDispatcher,
            EffectDispatcher,
            SystemStage::parallel(),
        );
    }
}
