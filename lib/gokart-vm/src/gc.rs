use std::collections::VecDeque;

use crate::{
    state::State,
    value::{Value, ValueBlock, ValueClosure, ValueEnv, RESERVED_TAG},
};

pub struct GC {
    threshold: usize,
}

impl GC {
    pub fn new(threshold: usize) -> Self {
        Self { threshold }
    }

    pub fn cleanup(&self, state: &mut State) {
        let mut vacuum = Vacuum::new();

        for i in 0..state.asp.ptr {
            match state.asp.data[i] {
                crate::state::StackValue::Ptr(p) => vacuum.mark_value(p),
                crate::state::StackValue::Mark => (),
            }
        }

        for i in 0..state.rsp.ptr {
            match state.rsp.data[i] {
                crate::state::StackValue::Ptr(p) => vacuum.mark_value(p),
                crate::state::StackValue::Mark => (),
            }
        }

        vacuum.mark_value(state.env as *mut Value);
        vacuum.mark_value(state.acc);
        vacuum.run();

        state.heap.clean();
    }
}

pub struct Vacuum {
    queue: VecDeque<*mut Value>,
}

impl Vacuum {
    pub fn new() -> Self {
        Vacuum {
            queue: VecDeque::new(),
        }
    }

    pub fn mark_value(&mut self, value: *mut Value) {
        if !value.is_null() && unsafe { &*value }.color() == 0 {
            self.queue.push_back(value);
        }
    }

    pub fn run(&mut self) {
        while let Some(v) = self.queue.pop_front() {
            let cur = unsafe { &mut *v };

            if cur.tag() == ValueClosure::tag() {
                let cur = cur.cast_mut::<ValueClosure>();
                self.mark_value(cur.env);
            } else if cur.tag() == ValueEnv::tag() {
                let cur = cur.cast_mut::<ValueEnv>();
                self.mark_value(cur.cur);
                self.mark_value(cur.env as *mut Value);
            } else if cur.tag() < RESERVED_TAG {
                let cur = cur.cast_mut::<ValueBlock>();
                for field in &cur.data {
                    self.mark_value(*field as *mut Value);
                }
            }

            cur.set_color(2);
        }
    }
}

impl Default for GC {
    fn default() -> Self {
        Self::new(10_000)
    }
}
