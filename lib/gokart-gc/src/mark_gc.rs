use std::collections::HashSet;

use gokart_core::{GHeap, GLabel, GRef, GValue, VMState, GGC};

use crate::{vacuum::Vacuum, RetainMarked, Trace};

pub struct MarkGC {
    threshold: usize,
}

impl<H, R, V, L> GGC<H, R, V, L> for MarkGC
where
    H: GHeap<R, V, L> + RetainMarked<R>,
    R: GRef,
    V: GValue<L> + Trace<R>,
    L: GLabel,
{
    fn is_necessary(&self, state: &VMState<H, R, V, L>) -> bool {
        state.heap.size() > self.threshold
    }

    fn cleanup(&self, state: &mut VMState<H, R, V, L>) {
        let mut vacuum = Vacuum::<R>::new();
        vacuum.mark(state.env);
        state.stack.iter().for_each(|&e| vacuum.mark(e));

        let mut marked = HashSet::<R>::new();
        while let Some(r) = vacuum.next() {
            if marked.insert(r) {
                state.heap[r].trace(&mut vacuum);
            }
        }
        state.heap.retain_marked(marked.into_iter());
    }
}
