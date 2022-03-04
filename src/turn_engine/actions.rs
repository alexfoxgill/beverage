use std::any::TypeId;
use std::collections::VecDeque;

use bevy_ecs::query::QueryEntityError;
use downcast_rs::*;
use dyn_clone::DynClone;

use super::{effects::EffectQueue, DynamicWrapper, InnerType};

pub trait Action: Downcast + DynClone + Send + Sync + std::fmt::Debug {
    fn cost(&self) -> u8;
}
impl_downcast!(Action);
dyn_clone::clone_trait_object!(Action);

#[derive(Debug, Clone)]
pub struct AnyAction(pub Box<dyn Action>);

impl AnyAction {
    pub fn cost(&self) -> u8 {
        self.0.cost()
    }
}

impl<A: Action> From<A> for AnyAction {
    fn from(action: A) -> Self {
        AnyAction(Box::new(action))
    }
}

impl<A: Action> DynamicWrapper<A> for AnyAction {
    fn downcast(self) -> Option<A> {
        let res = self.0.downcast::<A>().ok()?;
        Some(*res)
    }

    fn downcast_ref(&self) -> Option<&A> {
        self.0.downcast_ref()
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

impl<A: Action + Into<AnyAction>> Extend<A> for ActionQueue {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        for x in iter {
            self.push(x);
        }
    }
}

impl ActionQueue {
    pub fn new<A: Action + Into<AnyAction>>(action: A) -> ActionQueue {
        let mut queue = ActionQueue::default();
        queue.push(action);
        queue
    }

    pub fn pop(&mut self) -> Option<AnyAction> {
        self.0.pop_front()
    }

    pub fn push<A: Action + Into<AnyAction>>(&mut self, action: A) -> u8 {
        let cost = action.cost();
        self.0.push_back(action.into());
        cost
    }
}

pub type ActionResult = Result<EffectQueue, AnyActionError>;

pub trait ActionError: core::fmt::Debug {}

#[derive(Debug)]
pub struct AnyActionError(Box<dyn ActionError>);

impl AnyActionError {
    pub fn res_generic<T>(str: &str) -> Result<T, AnyActionError> {
        Err(Self::generic(str))
    }

    pub fn generic(str: &str) -> AnyActionError {
        AnyActionError(Box::new(GenericActionError(str.into())))
    }
}

#[derive(Debug)]
pub struct GenericActionError(String);
impl ActionError for GenericActionError {}

#[derive(Debug)]
pub struct QueryEntityActionError(QueryEntityError);
impl ActionError for QueryEntityActionError {}

impl From<QueryEntityError> for AnyActionError {
    fn from(e: QueryEntityError) -> Self {
        AnyActionError(Box::new(QueryEntityActionError(e)))
    }
}
