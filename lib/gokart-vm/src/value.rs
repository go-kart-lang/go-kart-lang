use gokart_gc;

pub type Ref = *mut gokart_gc::gokart_value;

#[repr(C)]
pub struct GValue<T> {
    pub inner: gokart_gc::gokart_value,
    pub data: T,
}

pub fn gvalue_cast<T>(ptr: Ref) -> &'static mut T {
    &mut unsafe { &mut *(ptr as *mut GValue<T>) }.data
}

pub fn get_tag(ptr: Ref) -> ValueTag {
    unsafe { std::mem::transmute(gokart_gc::gokart_get_tag(ptr)) }
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
