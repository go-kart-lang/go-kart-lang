use crate::value::{Ref, Value};
use std::{collections::HashMap, ops};

#[derive(Default, Debug)]
pub struct Heap {
    data: HashMap<Ref, Value>,
    next_ref: Ref,
}

impl Heap {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            next_ref: Ref::default(),
        }
    }

    pub fn alloc(&mut self, val: Value) -> Ref {
        let cur_ref = self.next_ref;
        self.next_ref += 1;
        self.data.insert(cur_ref, val);
        cur_ref
    }

    #[inline]
    pub fn retain<F>(&mut self, f: F)
    where
        F: Fn(Ref) -> bool,
    {
        self.data.retain(|&k, _| f(k));
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl ops::Index<Ref> for Heap {
    type Output = Value;

    #[inline]
    fn index(&self, r: Ref) -> &Self::Output {
        &self.data[&r]
    }
}
