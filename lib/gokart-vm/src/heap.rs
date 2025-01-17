use crate::value::{GValue, Ref, ValueTag};
use gokart_core::{Double, Int, Label, Str, Tag};

#[derive(Debug)]
pub struct Heap {
    heap: gokart_gc::Heap,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            heap: gokart_gc::Heap::new(),
        }
    }

    pub fn bytes_allocated(&self) -> u64 {
        self.heap.bytes_allocated()
    }

    pub fn objects_allocated(&self) -> u64 {
        self.heap.objects_allocated()
    }

    pub fn allocate<T>(
        &mut self,
        tag: ValueTag,
        data: T,
        finalizer: Option<gokart_gc::Finalizer>,
    ) -> Ref {
        let size = std::mem::size_of::<GValue<T>>();
        let ptr = self.heap.allocate(size as u64, finalizer);
        let v = unsafe { &mut *(ptr as *mut GValue<T>) };

        v.inner.header = (tag as u64) << 8;
        v.data = data;

        ptr
    }

    pub fn clean(&mut self) {
        self.heap.sweep();
    }

    pub fn allocate_int(&mut self, data: Int) -> Ref {
        self.allocate(ValueTag::IntTag, data, None)
    }

    pub fn allocate_double(&mut self, data: Double) -> Ref {
        self.allocate(ValueTag::DoubleTag, data, None)
    }

    pub fn allocate_str(&mut self, data: Str) -> Ref {
        self.allocate(
            ValueTag::StrTag,
            std::mem::ManuallyDrop::new(data),
            Some(str_finalizer),
        )
    }

    pub fn allocate_vector_int(&mut self, data: rpds::Vector<Int>) -> Ref {
        self.allocate(
            ValueTag::VectorInt,
            std::mem::ManuallyDrop::new(data),
            Some(vector_int_finalizer),
        )
    }

    pub fn allocate_label(&mut self, data: Label) -> Ref {
        self.allocate(ValueTag::Label, data, None)
    }

    pub fn allocate_pair(&mut self, lhs: Ref, rhs: Ref) -> Ref {
        self.allocate(ValueTag::Pair, (lhs, rhs), None)
    }

    pub fn allocate_tagged(&mut self, lhs: Tag, rhs: Ref) -> Ref {
        self.allocate(ValueTag::Tagged, (lhs, rhs), None)
    }

    pub fn allocate_closure(&mut self, lhs: Ref, rhs: Label) -> Ref {
        self.allocate(ValueTag::Closure, (lhs, rhs), None)
    }
}

#[no_mangle]
unsafe extern "C" fn str_finalizer(ptr: *mut gokart_gc::gokart_value) {
    let str = unsafe { &mut *(ptr as *mut GValue<std::mem::ManuallyDrop<Str>>) };

    std::mem::ManuallyDrop::drop(&mut str.data);
}

#[no_mangle]
unsafe extern "C" fn vector_int_finalizer(ptr: *mut gokart_gc::gokart_value) {
    let str = unsafe { &mut *(ptr as *mut GValue<std::mem::ManuallyDrop<rpds::Vector<Int>>>) };

    std::mem::ManuallyDrop::drop(&mut str.data);
}
