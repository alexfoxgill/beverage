use std::any::TypeId;
use std::collections::VecDeque;

use downcast_rs::*;

use super::{DynamicWrapper, InnerType};

pub trait Effect: Downcast + Send + Sync + std::fmt::Debug {}
impl_downcast!(Effect);

#[derive(Debug)]
pub struct AnyEffect(pub Box<dyn Effect>);

impl<T: Effect> DynamicWrapper<T> for AnyEffect {
    fn downcast(self) -> Option<T> {
        let res = self.0.downcast::<T>().ok()?;
        Some(*res)
    }
}

impl InnerType for AnyEffect {
    fn inner_type(&self) -> TypeId {
        (&*self.0).as_any().type_id()
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

    pub fn new<T: Effect + 'static>(effect: T) -> Self {
        Self::default().with(effect)
    }

    pub fn with<T: Effect + 'static>(mut self, effect: T) -> Self {
        self.0.push_back(AnyEffect(Box::new(effect)));
        self
    }

    pub fn append(&mut self, mut other: EffectQueue) {
        self.0.append(&mut other.0);
    }
}
