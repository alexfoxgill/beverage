use std::collections::VecDeque;

use super::EffectEvent;

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
