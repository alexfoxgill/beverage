use std::any::Any;

use std::collections::VecDeque;

#[derive(Default)]
pub struct EffectQueue(VecDeque<EffectEvent>);

impl EffectQueue {
    pub fn pop(&mut self) -> Option<EffectEvent> {
        self.0.pop_front()
    }

    pub fn push(&mut self, action: EffectEvent) {
        self.0.push_back(action);
    }
}

pub trait Effect: Send + Sync + std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct EffectEvent(pub Box<dyn Effect>);

impl EffectEvent {
    pub fn as_effect<T: Any + Effect>(&self) -> Option<&T> {
        self.0.as_any().downcast_ref::<T>()
    }
}
