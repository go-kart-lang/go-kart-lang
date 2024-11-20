use std::collections::VecDeque;

use gokart_core::GRef;

pub struct Vacuum<R>
where
    R: GRef,
{
    pending: VecDeque<R>,
}

impl<R> Vacuum<R>
where
    R: GRef,
{
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }

    pub fn mark(&mut self, r: R) {
        self.pending.push_back(r);
    }

    pub fn next(&mut self) -> Option<R> {
        self.pending.pop_front()
    }
}
