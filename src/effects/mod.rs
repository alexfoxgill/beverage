use std::any::Any;

use bevy::prelude::*;

use self::{
    energy_cost_effect::EnergyCostEffectPlugin, face_effect::FaceEffectPlugin,
    kill_effect::KillEffectPlugin, move_effect::MoveEffectPlugin,
};

pub mod energy_cost_effect;
pub mod face_effect;
pub mod kill_effect;
pub mod move_effect;

pub trait Effect: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct EffectEvent(Box<dyn Effect>);

impl EffectEvent {
    pub fn as_effect<T: Any + Effect>(&self) -> Option<&T> {
        self.0.as_any().downcast_ref::<T>()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
pub struct EffectProducer;

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
pub struct EffectOutcome;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EffectEvent>()
            .add_plugin(EnergyCostEffectPlugin)
            .add_plugin(FaceEffectPlugin)
            .add_plugin(KillEffectPlugin)
            .add_plugin(MoveEffectPlugin);
    }
}
