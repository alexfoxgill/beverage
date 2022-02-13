use std::any::{Any, TypeId};

use bevy::prelude::*;

use super::action_queue::{ActionDispatcherStage, ActionQueue, ActionSchedules};

#[derive(Debug)]
pub struct ActionEvent(pub Box<dyn Action>);

impl ActionEvent {
    pub fn as_action<T: Any + Action>(&self) -> Option<&T> {
        self.0.as_any().downcast_ref::<T>()
    }

    pub fn inner_type(&self) -> TypeId {
        self.0.as_any().type_id()
    }

    pub fn insert_resource(&self, world: &mut World) {
        self.0.insert_resource(world);
    }
}

pub trait Action: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn insert_resource(&self, world: &mut World);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, StageLabel)]
pub struct ActionDispatcher;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionSchedules>()
            .init_resource::<ActionQueue>()
            .add_stage_after(CoreStage::Update, ActionDispatcher, ActionDispatcherStage);
    }
}
