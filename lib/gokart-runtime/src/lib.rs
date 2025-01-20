mod rt;

use std::{
    alloc::{self, Layout},
    collections::{BTreeMap, VecDeque},
};

pub use rt::*;

#[no_mangle]
pub extern "C" fn gokart_get_tag(v: *mut rt::gokart_value) -> u64 {
    let v = unsafe { &*v };
    v.header >> 8
}

#[no_mangle]
pub extern "C" fn gokart_set_tag(v: *mut rt::gokart_value, tag: u64) {
    let v = unsafe { &mut *v };
    v.header = (tag << 8) + (v.header & 0xff);
}

#[no_mangle]
pub extern "C" fn gokart_get_color(v: *mut rt::gokart_value) -> u8 {
    let v = unsafe { &*v };
    (v.header & 0xff) as u8
}

#[no_mangle]
pub extern "C" fn gokart_set_color(v: *mut rt::gokart_value, color: u8) {
    let v = unsafe { &mut *v };
    v.header = (color as u64) + (v.header & 0xffffffffffffff00);
}

pub type Finalizer = unsafe extern "C" fn(ptr: *mut rt::gokart_value);

#[derive(Debug)]
#[repr(C)]
pub struct GcImpl {
    inner: rt::gokart_gc,
    finalizers: BTreeMap<*mut rt::gokart_value, Finalizer>,
    layouts: BTreeMap<usize, Layout>,
}

#[no_mangle]
pub extern "C" fn gokart_allocate(
    m_ptr: *mut rt::gokart_machine,
    size: u64,
    finalizer: ::std::option::Option<unsafe extern "C" fn(arg1: *mut rt::gokart_value)>,
) -> *mut rt::gokart_value {
    let m = unsafe { &mut *m_ptr };
    let gc = unsafe { &mut *(m.gc as *mut GcImpl) };

    let layout = Layout::from_size_align(size as usize, align_of::<*const u8>() * 2).unwrap();
    let ptr = unsafe { alloc::alloc(layout) as *mut rt::gokart_value };
    let v = unsafe { &mut *ptr };

    v.next = gc.inner.head;
    v.size = size;
    v.header = 0;
    gc.inner.head = ptr;
    gc.layouts.insert(ptr as usize, layout);

    gc.inner.bytes_allocated += size;
    gc.inner.objects_allocated += 1;

    if let Some(f) = finalizer {
        gc.finalizers.insert(ptr, f);
    }

    if gc.inner.objects_allocated >= gc.inner.objects_threshold {
        gokart_mark_sweep(m_ptr, ptr);
        // gc.inner.objects_threshold *= 2;
    }

    ptr
}

pub fn gvalue_cast<T>(ptr: Ref) -> &'static mut T {
    &mut unsafe { &mut *(ptr as *mut GValue<T>) }.data
}

pub fn get_tag(ptr: Ref) -> ValueTag {
    unsafe { std::mem::transmute(gokart_get_tag(ptr)) }
}

pub const RESERVED_TAG: u64 = 0xffff;

#[repr(u64)]
pub enum ValueTag {
    IntTag = RESERVED_TAG,
    DoubleTag,
    StrTag,
    VectorInt,
    Label,
    Pair,
    Tagged,
    Closure,
}

#[repr(C)]
pub struct GValue<T> {
    pub inner: rt::gokart_value,
    pub data: T,
}

#[no_mangle]
unsafe extern "C" fn vector_int_finalizer(ptr: *mut rt::gokart_value) {
    let vec = unsafe { &mut *(ptr as *mut GValue<std::mem::ManuallyDrop<rpds::Vector<i64>>>) };

    std::mem::ManuallyDrop::drop(&mut vec.data);
}

#[no_mangle]
pub extern "C" fn gokart_get_vector_int(ptr: *mut rt::gokart_value) -> *mut rpds::Vector<i64> {
    unsafe { &(*(ptr as *mut GValue<rpds::Vector<i64>>)).data as *const rpds::Vector<i64> as *mut rpds::Vector<i64> }
}

#[no_mangle]
pub extern "C" fn gokart_allocate_vector_int(m: *mut rt::gokart_machine) -> *mut rt::gokart_value {
    let ptr = gokart_allocate(
        m,
        std::mem::size_of::<GValue<std::mem::ManuallyDrop<rpds::Vector<i64>>>>() as u64,
        Some(vector_int_finalizer),
    );
    gokart_set_tag(ptr, ValueTag::VectorInt as u64);

    unsafe { std::ptr::write(gokart_get_vector_int(ptr), rpds::Vector::new()) };


    ptr
}

#[no_mangle]
pub extern "C" fn gokart_allocate_int(m_ptr: *mut rt::gokart_machine, data: i64) -> Ref {
    let ptr = gokart_allocate(m_ptr, std::mem::size_of::<GValue<i64>>() as u64, None);
    let vec = unsafe { &mut *(ptr as *mut GValue<i64>) };
    vec.data = data;
    gokart_set_tag(ptr, ValueTag::IntTag as u64);

    ptr
}

#[no_mangle]
pub extern "C" fn gokart_allocate_double(m_ptr: *mut rt::gokart_machine, data: f64) -> Ref {
    let ptr = gokart_allocate(m_ptr, std::mem::size_of::<GValue<f64>>() as u64, None);
    let vec = unsafe { &mut *(ptr as *mut GValue<f64>) };
    vec.data = data;
    gokart_set_tag(ptr, ValueTag::DoubleTag as u64);

    ptr
}

pub type Label = u64;
pub type Tag = u64;

#[no_mangle]
pub extern "C" fn gokart_allocate_label(m_ptr: *mut rt::gokart_machine, data: Label) -> Ref {
    let ptr = gokart_allocate(m_ptr, std::mem::size_of::<GValue<Label>>() as u64, None);
    let vec = unsafe { &mut *(ptr as *mut GValue<Label>) };
    vec.data = data;
    gokart_set_tag(ptr, ValueTag::Label as u64);

    ptr
}

#[no_mangle]
pub extern "C" fn gokart_allocate_pair(m_ptr: *mut rt::gokart_machine, lhs: Ref, rhs: Ref) -> Ref {
    let ptr = gokart_allocate(
        m_ptr,
        std::mem::size_of::<GValue<(Ref, Ref)>>() as u64,
        None,
    );
    let vec = unsafe { &mut *(ptr as *mut GValue<(Ref, Ref)>) };
    vec.data.0 = lhs;
    vec.data.1 = rhs;
    gokart_set_tag(ptr, ValueTag::Pair as u64);

    ptr
}

#[no_mangle]
pub extern "C" fn gokart_allocate_tagged(
    m_ptr: *mut rt::gokart_machine,
    lhs: Tag,
    rhs: Ref,
) -> Ref {
    let ptr = gokart_allocate(
        m_ptr,
        std::mem::size_of::<GValue<(Tag, Ref)>>() as u64,
        None,
    );
    let vec = unsafe { &mut *(ptr as *mut GValue<(Tag, Ref)>) };
    vec.data.0 = lhs;
    vec.data.1 = rhs;
    gokart_set_tag(ptr, ValueTag::Tagged as u64);

    ptr
}

#[no_mangle]
pub extern "C" fn gokart_allocate_closure(
    m_ptr: *mut rt::gokart_machine,
    lhs: Ref,
    rhs: Label,
) -> Ref {
    let ptr = gokart_allocate(
        m_ptr,
        std::mem::size_of::<GValue<(Ref, Label)>>() as u64,
        None,
    );
    let vec = unsafe { &mut *(ptr as *mut GValue<(Ref, Label)>) };
    vec.data.0 = lhs;
    vec.data.1 = rhs;
    gokart_set_tag(ptr, ValueTag::Closure as u64);

    ptr
}

#[no_mangle]
pub extern "C" fn gokart_allocate_string(
    m_ptr: *mut rt::gokart_machine,
    size: u64,
    s_ptr: *mut u8,
) -> *mut rt::gokart_value {
    let struct_size = std::mem::size_of::<GValue<u64>>() as u64;
    let ptr = gokart_allocate(m_ptr, struct_size + size, None);
    gokart_set_tag(ptr, ValueTag::StrTag as u64);
    *gvalue_cast::<u64>(ptr) = size;

    unsafe {
        std::ptr::copy_nonoverlapping(
            s_ptr,
            (ptr as *mut u8).byte_add(struct_size as usize),
            size as usize,
        )
    };

    ptr
}

#[no_mangle]
pub extern "C" fn gokart_sweep(m: *mut rt::gokart_machine) {
    let gc = unsafe { &mut *((&*m).gc as *mut GcImpl) };
    let mut prev = std::ptr::null_mut();
    let mut cur = gc.inner.head;

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
                gc.inner.head = cur;
            } else {
                unsafe { &mut *prev }.next = cur;
            }

            gc.inner.bytes_allocated -= unsafe { &*unreached }.size;
            gc.inner.objects_allocated -= 1;

            if let Some(f) = gc.finalizers.remove(&unreached) {
                unsafe { f(unreached) };
            }

            let layout = gc
                .layouts
                .remove(&(unreached as usize))
                .expect("trying to free a buffer not allocated");
            unsafe { alloc::dealloc(unreached as *mut u8, layout) };
        }
    }
}

pub type Ref = *mut rt::gokart_value;

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
        if !r.is_null() && gokart_get_color(r) == 0 {
            self.pending.push_back(r);
        }
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
                    let (_, rhs) = gvalue_cast::<(u64, Ref)>(v);
                    self.mark(*rhs);
                }
                ValueTag::Closure => {
                    let (lhs, _) = gvalue_cast::<(Ref, u64)>(v);
                    self.mark(*lhs);
                }
                _ => (),
            }

            gokart_set_color(v, 2);
        }
    }
}

#[no_mangle]
pub extern "C" fn gokart_mark_sweep(m_ptr: *mut rt::gokart_machine, tmp: *mut rt::gokart_value) {
    let m = unsafe { &mut *m_ptr };

    let mut vacuum = Vacuum::new();

    vacuum.mark(tmp);
    vacuum.mark(m.env);

    for i in 0..m.stack.length {
        vacuum.mark(unsafe { std::ptr::read(m.stack.data.add(i as usize)) });
    }

    vacuum.run();

    gokart_sweep(m_ptr);
}

fn gokart_stack_grow(m: *mut rt::gokart_machine) {
    let m = unsafe { &mut *m };
    let (new_cap, new_layout) = if m.stack.capacity == 0 {
        (1, Layout::array::<*mut rt::gokart_value>(1).unwrap())
    } else {
        let new_cap = 2 * m.stack.capacity as usize;
        let new_layout = Layout::array::<*mut rt::gokart_value>(new_cap).unwrap();
        (new_cap, new_layout)
    };

    let new_ptr = if m.stack.capacity == 0 {
        unsafe { alloc::alloc(new_layout) }
    } else {
        let old_layout = Layout::array::<*mut rt::gokart_value>(m.stack.capacity as usize).unwrap();
        let old_ptr = m.stack.data as *mut u8;
        unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
    };
    m.stack.data = new_ptr as *mut *mut rt::gokart_value;
    m.stack.capacity = new_cap as u64;
}

#[no_mangle]
pub extern "C" fn gokart_stack_push(m: *mut rt::gokart_machine, v: *mut rt::gokart_value) {
    let m = unsafe { &mut *m };
    if m.stack.length == m.stack.capacity {
        gokart_stack_grow(m);
    }

    unsafe {
        std::ptr::write(m.stack.data.add(m.stack.length as usize), v);
    }

    m.stack.length += 1;
}

#[no_mangle]
pub extern "C" fn gokart_stack_peek(m: *mut rt::gokart_machine) -> *mut rt::gokart_value {
    let m = unsafe { &mut *m };
    if m.stack.length == 0 {
        std::ptr::null_mut()
    } else {
        unsafe { std::ptr::read(m.stack.data.add((m.stack.length - 1) as usize)) }
    }
}

#[no_mangle]
pub extern "C" fn gokart_stack_pop(m: *mut rt::gokart_machine) -> *mut rt::gokart_value {
    let m = unsafe { &mut *m };
    if m.stack.length == 0 {
        std::ptr::null_mut()
    } else {
        m.stack.length -= 1;
        unsafe { std::ptr::read(m.stack.data.add(m.stack.length as usize)) }
    }
}

#[no_mangle]
pub extern "C" fn gokart_machine_init() -> *mut rt::gokart_machine {
    let machine_layout = Layout::new::<rt::gokart_machine>();
    let gc_layout = Layout::new::<GcImpl>();

    let machine_ptr = unsafe { alloc::alloc(machine_layout) as *mut rt::gokart_machine };
    let gc_ptr = unsafe { alloc::alloc(gc_layout) as *mut GcImpl };

    unsafe {
        std::ptr::write(
            gc_ptr,
            GcImpl {
                inner: rt::gokart_gc {
                    bytes_allocated: 0,
                    bytes_threshold: 0,
                    head: std::ptr::null_mut(),
                    objects_threshold: 0,
                    objects_allocated: 0,
                },
                finalizers: BTreeMap::new(),
                layouts: BTreeMap::new(),
            },
        );

        std::ptr::write(
            machine_ptr,
            rt::gokart_machine {
                env: std::ptr::null_mut(),
                gc: gc_ptr as *mut rt::gokart_gc,
                ip: 0,
                is_running: 1,
                stack: rt::gokart_stack {
                    capacity: 0,
                    length: 0,
                    data: std::ptr::null_mut()
                }
            }
        )
    };

    machine_ptr
}

#[no_mangle]
pub extern "C" fn gokart_machine_free(m: *mut rt::gokart_machine) {
    let machine_layout = Layout::new::<rt::gokart_machine>();
    let gc_layout = Layout::new::<GcImpl>();

    let machine = unsafe { &mut *m };

    if machine.stack.capacity != 0 {
        let layout =
            Layout::array::<*mut rt::gokart_value>(machine.stack.capacity as usize).unwrap();
        unsafe {
            alloc::dealloc(machine.stack.data as *mut u8, layout);
        }
    }

    gokart_sweep(m);

    assert_eq!(unsafe { &*machine.gc }.head, std::ptr::null_mut());

    unsafe { alloc::dealloc(machine.gc as *mut u8, gc_layout) };
    unsafe { alloc::dealloc(m as *mut u8, machine_layout) };
}
