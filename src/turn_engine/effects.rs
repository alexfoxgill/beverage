use std::any::TypeId;
use std::collections::VecDeque;

use downcast_rs::*;
use dyn_clone::*;

use super::{DynamicWrapper, InnerType};

pub trait Effect: Downcast + DynClone + Send + Sync + std::fmt::Debug {}
downcast_rs::impl_downcast!(Effect);
dyn_clone::clone_trait_object!(Effect);

#[derive(Debug, Clone)]
pub struct AnyEffect(pub Box<dyn Effect>);

impl<E: Effect> From<E> for AnyEffect {
    fn from(effect: E) -> Self {
        AnyEffect(Box::new(effect))
    }
}

impl<E: Effect> DynamicWrapper<E> for AnyEffect {
    fn downcast(self) -> Option<E> {
        let res = self.0.downcast::<E>().ok()?;
        Some(*res)
    }

    fn downcast_ref(&self) -> Option<&E> {
        self.0.downcast_ref::<E>()
    }
}

impl InnerType for AnyEffect {
    fn inner_type(&self) -> TypeId {
        (&*self.0).as_any().type_id()
    }
}

#[derive(Default)]
pub struct EffectQueue(VecDeque<AnyEffect>);

impl<E, const N: usize> From<[E; N]> for EffectQueue
where
    EffectQueue: Extend<E>,
{
    fn from(arr: [E; N]) -> Self {
        let mut queue = EffectQueue::default();
        queue.extend(arr);
        queue
    }
}

impl<E: Into<AnyEffect>> Extend<E> for EffectQueue {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        for x in iter {
            self.push(x.into())
        }
    }
}

impl EffectQueue {
    pub fn pop(&mut self) -> Option<AnyEffect> {
        self.0.pop_front()
    }

    pub fn push<T: Into<AnyEffect>>(&mut self, effect: T) {
        self.0.push_back(effect.into());
    }

    pub fn new<T: Into<AnyEffect>>(effect: T) -> Self {
        Self::default().then(effect)
    }

    pub fn then<T: Into<AnyEffect>>(mut self, effect: T) -> Self {
        self.0.push_back(effect.into());
        self
    }

    pub fn append(&mut self, mut other: EffectQueue) {
        self.0.append(&mut other.0);
    }

    pub fn find<E: Effect>(&self) -> Option<&E> {
        self.0.iter().find_map(|e| e.downcast_ref())
    }
}
