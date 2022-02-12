pub mod attack_action;
pub mod end_turn_action;
pub mod move_action;
pub mod rotate_action;

use std::any::Any;

use bevy::prelude::*;

use crate::effects::EffectEvent;

use self::{
    attack_action::AttackActionPlugin, end_turn_action::EndTurnActionPlugin,
    move_action::MoveActionPlugin, rotate_action::RotateActionPlugin,
};

use super::common::*;

#[derive(Debug)]
pub struct ActionEvent(Box<dyn Action>);

impl ActionEvent {
    pub fn as_action<T: Any + Action>(&self) -> Option<&T> {
        self.0.as_any().downcast_ref::<T>()
    }
}

pub trait Action: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn entity(&self) -> Entity;
    fn effects(&self) -> Vec<EffectEvent>;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
pub struct ActionInterpreter;

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemLabel)]
pub struct ActionProducer;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_plugin(AttackActionPlugin)
            .add_plugin(EndTurnActionPlugin)
            .add_plugin(MoveActionPlugin)
            .add_plugin(RotateActionPlugin);
    }
}
