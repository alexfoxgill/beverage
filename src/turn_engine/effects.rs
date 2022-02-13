use std::any::{Any, TypeId};

use std::collections::VecDeque;

use bevy::prelude::World;

pub trait Effect: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn insert_resource(&self, world: &mut World);
}

#[derive(Debug)]
pub struct EffectEvent(pub Box<dyn Effect>);

impl EffectEvent {
    pub fn as_effect<T: Any + Effect>(&self) -> Option<&T> {
        self.0.as_any().downcast_ref::<T>()
    }

    pub fn inner_type(&self) -> TypeId {
        self.0.as_any().type_id()
    }

    pub fn insert_resource(&self, world: &mut World) {
        self.0.insert_resource(world);
    }
}

#[derive(Default)]
pub struct EffectQueue(pub VecDeque<EffectEvent>);

impl EffectQueue {
    pub fn pop(&mut self) -> Option<EffectEvent> {
        self.0.pop_front()
    }

    pub fn push(&mut self, action: EffectEvent) {
        self.0.push_back(action);
    }
}
