use std::collections::VecDeque;

use gokart_core::GRef;

use crate::marker::Marker;

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

    pub fn next(&mut self) -> Option<R> {
        self.pending.pop_front()
    }
}

impl<R> Marker<R> for &mut Vacuum<R>
where
    R: GRef,
{
    fn mark(self, r: R) {
        self.pending.push_back(r);
    }
}
