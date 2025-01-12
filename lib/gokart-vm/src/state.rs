use crate::{
    heap::Heap,
    value::{Ref, Value},
};
use core::slice;
use gokart_core::Label;

#[derive(Debug)]
pub struct State {
    pub ip: Label,
    pub is_running: bool,
    pub heap: Heap,
    pub env: Ref,
    pub stack: Stack<Ref>,
}

impl State {
    pub fn init_with<F>(f: F) -> Self
    where
        F: Fn(&mut Heap) -> Ref,
    {
        let mut heap = Heap::default();
        let env = f(&mut heap);
        Self {
            ip: 0,
            is_running: true,
            heap,
            env,
            stack: Stack::new(),
        }
    }

    #[inline]
    pub fn cur_env(&self) -> &Value {
        &self.heap[self.env]
    }

    #[inline]
    pub fn alloc(&mut self, val: Value) -> Ref {
        self.heap.alloc(val)
    }
}

impl Default for State {
    fn default() -> Self {
        State::init_with(|h| h.alloc(Value::Empty))
    }
}

#[derive(Default, Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    #[inline]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> T {
        self.data.pop().unwrap()
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.data.iter()
    }
}
