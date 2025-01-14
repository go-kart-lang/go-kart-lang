use crate::{heap::Heap, value::{Value, ValueEnv}};
use gokart_core::Label;


#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub enum StackValue {
    Ptr(*mut Value),
    #[default] Mark
}

impl StackValue {
    pub fn as_ptr(self) -> *mut Value {
        match self {
            StackValue::Ptr(p) => p,
            StackValue::Mark => panic!("expected ptr"),
        }
    }
}

#[derive(Debug)]
pub struct State {
    pub ip: usize,
    pub is_running: bool,
    pub heap: Heap,
    pub env: *mut ValueEnv,
    pub acc: *mut Value,
    pub rsp: Stack<10_000, StackValue>,
    pub asp: Stack<10_000, StackValue>,
}

impl State {

    pub fn new() -> Self {
        Self {
            ip: 0,
            is_running: true,
            heap: Heap::new(),
            env: std::ptr::null_mut(),
            acc: std::ptr::null_mut(),
            rsp: Stack::new(),
            asp: Stack::new()
        }
    }
}


#[derive(Debug)]
pub struct Stack<const N: usize, T> {
    pub data: [T; N],
    pub ptr: usize
}

impl <const N: usize, T: Default + Copy> Stack<N, T>  {

    pub fn new() -> Self {
        Stack { data: [T::default(); N], ptr: 0 }
    }

    pub fn push(&mut self, v: T) {
        self.data[self.ptr] = v;
        self.ptr += 1;
    }

    pub fn pop(&mut self) -> T {
        self.ptr -= 1;
        self.data[self.ptr]
    }

    pub fn access(&self, n: usize) -> T {
        self.data[self.ptr - n - 1]
    }
}
