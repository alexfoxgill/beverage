use std::any::TypeId;
use std::collections::VecDeque;

use downcast_rs::*;

use super::{DynamicWrapper, InnerType};

pub trait Action: Downcast + Send + Sync + std::fmt::Debug {}
impl_downcast!(Action);

#[derive(Debug)]
pub struct AnyAction(pub Box<dyn Action>);

impl<A: Action> From<A> for AnyAction {
    fn from(action: A) -> Self {
        AnyAction(Box::new(action))
    }
}

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
pub struct ActionQueue(VecDeque<AnyAction>);

impl<A: Action + 'static, const N: usize> From<[A; N]> for ActionQueue {
    fn from(arr: [A; N]) -> Self {
        let mut queue = ActionQueue::default();
        queue.extend(arr);
        queue
    }
}

impl<A: Into<AnyAction>> Extend<A> for ActionQueue {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for x in iter {
            self.push(x)
        }
    }
}

impl ActionQueue {
    pub fn new<A: Into<AnyAction>>(action: A) -> ActionQueue {
        let mut queue = ActionQueue::default();
        queue.push(action);
        queue
    }

    pub fn pop(&mut self) -> Option<AnyAction> {
        self.0.pop_front()
    }

    pub fn push<A: Into<AnyAction>>(&mut self, action: A) {
        self.0.push_back(action.into());
    }
}
