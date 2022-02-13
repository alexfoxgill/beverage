use std::any::TypeId;

use std::collections::VecDeque;

use bevy::prelude::World;
use downcast_rs::*;

pub trait Effect: Downcast + Send + Sync + std::fmt::Debug {
    fn insert_resource(&self, world: &mut World);
}
impl_downcast!(Effect);

#[derive(Debug)]
pub struct AnyEffect(pub Box<dyn Effect>);

impl AnyEffect {
    pub fn inner_type(&self) -> TypeId {
        (&*self.0).as_any().type_id()
    }

    pub fn insert_resource(&self, world: &mut World) {
        self.0.insert_resource(world);
    }
}

#[derive(Default)]
pub struct EffectQueue(pub VecDeque<AnyEffect>);

impl EffectQueue {
    pub fn pop(&mut self) -> Option<AnyEffect> {
        self.0.pop_front()
    }

    pub fn push<T: Effect + 'static>(&mut self, effect: T) {
        self.0.push_back(AnyEffect(Box::new(effect)));
    }
}
