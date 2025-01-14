use gokart_core::{Double, Int, Label, Str, Tag};

#[repr(C)]
#[derive(Debug)]
pub struct Value {
    header: u64,
    pub next: *mut Value,
}

pub const RESERVED_TAG: u64 = 0xffff;

impl Value {
    pub fn new(tag: u64, color: u8, next: *mut Value) -> Self {
        Value {
            header: (tag << 8) + (color as u64),
            next,
        }
    }

    pub fn tag(&self) -> u64 {
        self.header >> 8
    }

    pub fn set_tag(&mut self, tag: u64) {
        self.header = (tag << 8) | (self.header & 0xff);
    }

    pub fn color(&self) -> u8 {
        (self.header & 0xff) as u8
    }

    pub fn set_color(&mut self, color: u8) {
        self.header = (self.header & 0xffffffffffffff00) | (color as u64)
    }

    pub fn set_next(&mut self, ptr: *mut Value) {
        self.next = ptr
    }

    pub fn cast<T>(&self) -> &T {
        unsafe { &*(self as *const Value as *const T) }
    }

    pub fn cast_mut<T>(&mut self) -> &mut T {
        unsafe { &mut *(self as *mut Value as *mut T) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ValueInt {
    value: Value,
    pub data: i64,
}

impl ValueInt {
    pub fn new(data: i64) -> impl FnOnce(*mut Self) -> () {
        move |v| {
            let r = unsafe { &mut *v };
            r.value.set_tag(ValueInt::tag());
            r.data = data
        }
    }

    pub fn tag() -> u64 {
        RESERVED_TAG + 1
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ValueClosure {
    value: Value,
    pub lbl: Label,
    pub env: *mut Value,
}

impl ValueClosure {
    pub fn new(lbl: Label, env: *mut Value) -> impl FnOnce(*mut Self) -> () {
        move |v| {
            let r = unsafe { &mut *v };
            r.value.set_tag(ValueClosure::tag());
            r.env = env;
            r.lbl = lbl;
        }
    }

    pub fn tag() -> u64 {
        RESERVED_TAG + 2
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ValueEnv {
    value: Value,
    pub cur: *mut Value,
    pub env: *mut ValueEnv,
}

impl ValueEnv {
    pub fn new(cur: *mut Value, env: *mut ValueEnv) -> impl FnOnce(*mut Self) -> () {
        move |v| {
            let r = unsafe { &mut *v };
            r.value.set_tag(ValueEnv::tag());
            r.cur = cur;
            r.env = env;
        }
    }

    pub fn new0(cur: *mut Value) -> impl FnOnce(*mut Self) -> () {
        ValueEnv::new(cur, std::ptr::null_mut())
    }

    pub fn access(env: *mut ValueEnv, n: usize) -> *mut Value {
        let mut env1 = env;

        for _ in 1..n {
            env1 = unsafe { &*env }.env;
        }

        unsafe { &*env1 }.cur
    }

    pub fn tag() -> u64 {
        RESERVED_TAG + 3
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct ValueBlock {
    value: Value,
    pub data: Box<[*const Value]>,
}

impl ValueBlock {
    pub fn new(tag: u64, data: Box<[*const Value]>) -> impl FnOnce(*mut Self) -> () {
        move |v| {
            let r = unsafe { &mut *v };
            r.value.set_tag(tag);
            r.data = data;
        }
    }
}
