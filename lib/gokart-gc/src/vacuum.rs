use super::Heap;
use crate::heap::HeapRef;
use std::cell::RefCell;
use std::collections::BTreeSet;

pub struct Vacuum<'a> {
    heap: &'a mut Heap,
    pending: RefCell<BTreeSet<usize>>,
}

impl<'a> Vacuum<'a> {
    pub(super) fn new(heap: &'a mut Heap) -> Self {
        Vacuum {
            heap,
            pending: RefCell::new(BTreeSet::new()),
        }
    }

    pub fn mark<T>(&self, id: HeapRef<T>) {
        self.pending.borrow_mut().insert(id.id);
    }

    pub fn finish(self) {
        let mut marked = BTreeSet::new();
        loop {
            let tmp = self.pending.borrow_mut().pop_first();
            if let Some(id) = tmp {
                if marked.insert(id) {
                    self.heap.trace_item(id, &self);
                }
            } else {
                break;
            }
        }
        self.heap.retain_marked(&marked);
    }
}
