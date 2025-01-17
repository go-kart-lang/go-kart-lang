mod gc;

use std::{collections::HashMap, hash::Hash};

pub use gc::{gokart_heap, gokart_value};
use libc;

#[no_mangle]
pub extern "C" fn gokart_get_tag(v: *mut gc::gokart_value) -> u64 {
    let v = unsafe { &*v };
    v.header >> 8
}

#[no_mangle]
pub extern "C" fn gokart_set_tag(v: *mut gc::gokart_value, tag: u64) {
    let v = unsafe { &mut *v };
    v.header = (tag << 8) + (v.header & 0xff);
}

#[no_mangle]
pub extern "C" fn gokart_get_color(v: *mut gc::gokart_value) -> u8 {
    let v = unsafe { &*v };
    (v.header & 0xff) as u8
}

#[no_mangle]
pub extern "C" fn gokart_set_color(v: *mut gc::gokart_value, color: u8) {
    let v = unsafe { &mut *v };
    v.header = (color as u64) + (v.header & 0xffffffffffffff00);
}

pub type Finalizer = unsafe extern "C" fn(ptr: *mut gc::gokart_value);

#[derive(Debug)]
pub struct Heap {
    inner: gc::gokart_heap,
    finalizers: HashMap<*mut gc::gokart_value, Finalizer>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            inner: gc::gokart_heap {
                head: std::ptr::null_mut(),
                bytes_allocated: 0,
                objects_allocated: 0,
            },
            finalizers: HashMap::new(),
        }
    }

    pub fn bytes_allocated(&self) -> u64 {
        self.inner.bytes_allocated
    }

    pub fn objects_allocated(&self) -> u64 {
        self.inner.objects_allocated
    }

    pub fn allocate(&mut self, size: u64, finalizer: Option<Finalizer>) -> *mut gc::gokart_value {
        gokart_allocate(self as *mut Heap as *mut gc::gokart_heap, size, finalizer)
    }

    pub fn sweep(&mut self) {
        gokart_sweep(self as *mut Heap as *mut gc::gokart_heap)
    }
}

#[no_mangle]
pub extern "C" fn gokart_allocate(
    h: *mut gc::gokart_heap,
    size: u64,
    finalizer: Option<Finalizer>,
) -> *mut gc::gokart_value {
    let ptr = unsafe { libc::malloc(size as usize) as *mut gc::gokart_value };
    let v = unsafe { &mut *ptr };
    let h = unsafe { &mut *(h as *mut Heap) };
    v.next = h.inner.head;
    v.size = size;
    h.inner.head = ptr;

    h.inner.bytes_allocated += size;
    h.inner.objects_allocated += 1;

    if let Some(f) = finalizer {
        h.finalizers.insert(ptr, f);
    }

    ptr
}

#[no_mangle]
pub extern "C" fn gokart_sweep(h: *mut gc::gokart_heap) {
    let heap = unsafe { &mut *(h as *mut Heap) };
    let mut prev = std::ptr::null_mut();
    let mut cur = heap.inner.head;

    loop {
        if cur.is_null() {
            break;
        }
        let cur_o = unsafe { &mut *cur };

        if gokart_get_color(cur_o) == 2 {
            gokart_set_color(cur, 0);
            prev = cur;
            cur = cur_o.next;
        } else {
            let unreached = cur;
            cur = cur_o.next;
            if prev.is_null() {
                heap.inner.head = cur;
            } else {
                unsafe { &mut *prev }.next = cur;
            }

            heap.inner.bytes_allocated -= unsafe { &*unreached }.size;
            heap.inner.objects_allocated -= 1;

            if let Some(f) = heap.finalizers.remove(&unreached) {
                unsafe { f(unreached) };
            }

            unsafe { libc::free(unreached as *mut libc::c_void) };
        }
    }
}
