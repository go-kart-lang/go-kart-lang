use crate::{heap::Heap, value::Ref};
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
    pub fn new() -> Self {
        let heap = Heap::new();
        Self {
            ip: 0,
            is_running: true,
            heap,
            env: std::ptr::null_mut(),
            stack: Stack::new(),
        }
    }
}

#[derive(Default, Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(10_000),
        }
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

    pub fn clear(&mut self) {
        self.data.clear()
    }
}
