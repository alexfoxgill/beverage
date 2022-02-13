use std::any::TypeId;
use std::collections::VecDeque;

use bevy::prelude::*;
use downcast_rs::*;

#[derive(Debug)]
pub struct AnyAction(pub Box<dyn Action>);

impl AnyAction {
    pub fn inner_type(&self) -> TypeId {
        (&*self.0).as_any().type_id()
    }

    pub fn insert_resource(&self, world: &mut World) {
        self.0.insert_resource(world);
    }
}

pub trait Action: Downcast + Send + Sync + std::fmt::Debug {
    fn insert_resource(&self, world: &mut World);
}
impl_downcast!(Action);

#[derive(Default)]
pub struct ActionQueue(pub VecDeque<AnyAction>);

impl ActionQueue {
    pub fn pop(&mut self) -> Option<AnyAction> {
        self.0.pop_front()
    }

    pub fn push<T: Action + 'static>(&mut self, action: T) {
        self.0.push_back(AnyAction(Box::new(action)));
    }
}
