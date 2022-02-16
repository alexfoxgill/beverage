use std::any::TypeId;
use std::collections::VecDeque;

use downcast_rs::*;

use super::{DynamicWrapper, InnerType};

pub trait Action: Downcast + Send + Sync + std::fmt::Debug {}
impl_downcast!(Action);

#[derive(Debug)]
pub struct AnyAction(pub Box<dyn Action>);

impl<T: Action> DynamicWrapper<T> for AnyAction {
    fn downcast(self) -> Option<T> {
        let res = self.0.downcast::<T>().ok()?;
        Some(*res)
    }
}
impl InnerType for AnyAction {
    fn inner_type(&self) -> TypeId {
        (&*self.0).as_any().type_id()
    }
}

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
