use std::any::{Any, TypeId};
use std::collections::VecDeque;

use bevy::prelude::*;

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

#[derive(Default)]
pub struct ActionQueue(pub VecDeque<ActionEvent>);

impl ActionQueue {
    pub fn pop(&mut self) -> Option<ActionEvent> {
        self.0.pop_front()
    }

    pub fn push<T: Action + 'static>(&mut self, action: T) {
        self.0.push_back(ActionEvent(Box::new(action)));
    }
}
