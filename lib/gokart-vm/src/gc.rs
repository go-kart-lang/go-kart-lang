use std::collections::VecDeque;

use gokart_core::{Label, Tag};

use crate::{
    state::State,
    value::{get_tag, gvalue_cast, Ref, ValueTag},
};

pub struct GC {
    threshold_bytes: u64,
    threshold_objects: u64,
}

impl GC {
    #[inline]
    pub fn new(threshold_bytes: u64, threshold_objects: u64) -> Self {
        Self {
            threshold_bytes,
            threshold_objects,
        }
    }

    #[inline]
    pub fn is_necessary(&self, state: &State) -> bool {
        state.heap.bytes_allocated() > self.threshold_bytes
            || state.heap.objects_allocated() > self.threshold_objects
    }

    #[inline]
    pub fn cleanup(&self, state: &mut State) {
        let mut vacuum = Vacuum::new();
        vacuum.mark(state.env);
        state.stack.iter().for_each(|&e| vacuum.mark(e));

        vacuum.run();

        state.heap.clean();
    }
}

impl Default for GC {
    fn default() -> Self {
        Self::new(8 * 1024 * 1024, 100_000)
    }
}

struct Vacuum {
    pending: VecDeque<Ref>,
}

impl Vacuum {
    #[inline]
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
        }
    }

    #[inline]
    pub fn mark(&mut self, r: Ref) {
        if !r.is_null() && gokart_gc::gokart_get_color(r) == 0 {
            self.pending.push_back(r);
        }
    }

    #[inline]
    fn next(&mut self) -> Option<Ref> {
        self.pending.pop_front()
    }

    pub fn run(&mut self) {
        while let Some(v) = self.pending.pop_front() {
            match get_tag(v) {
                ValueTag::Pair => {
                    let (lhs, rhs) = gvalue_cast::<(Ref, Ref)>(v);
                    self.mark(*lhs);
                    self.mark(*rhs);
                }
                ValueTag::Tagged => {
                    let (_, rhs) = gvalue_cast::<(Tag, Ref)>(v);
                    self.mark(*rhs);
                }
                ValueTag::Closure => {
                    let (lhs, _) = gvalue_cast::<(Ref, Label)>(v);
                    self.mark(*lhs);
                }
                _ => (),
            }

            gokart_gc::gokart_set_color(v, 2);
        }
    }
}
