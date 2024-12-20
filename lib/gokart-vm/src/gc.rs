use std::collections::{HashSet, VecDeque};

use gokart_core::{Ref, Value};

use crate::state::State;

pub struct GC {
    threshold: usize,
}

impl GC {
    #[inline]
    pub fn new(threshold: usize) -> Self {
        Self { threshold }
    }

    #[inline]
    pub fn is_necessary(&self, state: &State) -> bool {
        state.heap.len() > self.threshold
    }

    #[inline]
    pub fn cleanup(&self, state: &mut State) {
        let mut vacuum = Vacuum::new();
        vacuum.mark(state.env);
        state.stack.iter().for_each(|&e| vacuum.mark(e));

        let mut marked = HashSet::<Ref>::new();
        loop {
            let next = vacuum.next();
            if let Some(r) = next {
                if marked.insert(r) {
                    state.heap[r].trace(&mut vacuum);
                }
            } else {
                break;
            }
        }
        state.heap.retain(|r| marked.contains(&r));
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
        self.pending.push_back(r);
    }

    #[inline]
    pub fn next(&mut self) -> Option<Ref> {
        self.pending.pop_front()
    }
}

trait Trace {
    fn trace(&self, vac: &mut Vacuum);
}

impl Trace for Value {
    fn trace(&self, vac: &mut Vacuum) {
        match *self {
            Value::Empty => (),
            Value::Int(_) => (),
            Value::Label(_) => (),
            Value::Pair(a, b) => {
                vac.mark(a);
                vac.mark(b);
            }
            Value::Tagged(_, r) => vac.mark(r),
            Value::Closure(r, _) => vac.mark(r),
            Value::CClosure(_) => (),
        }
    }
}
