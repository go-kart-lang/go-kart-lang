use crate::value::Value;

#[derive(Debug)]
pub struct Heap {
    pub data: *mut Value,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            data: std::ptr::null_mut(),
        }
    }

    pub fn allocate<T, F>(&mut self, initializer: F) -> *mut T
    where
        F: FnOnce(*mut T) -> (),
    {
        let ptr: *mut T = unsafe { libc::malloc(std::mem::size_of::<T>()) as *mut T };
        initializer(ptr);

        let v = unsafe { &mut *(ptr as *mut Value) };
        v.set_next(self.data);

        self.data = ptr as *mut Value;

        ptr
    }

    pub fn clean(&mut self) {
        let mut prev = std::ptr::null_mut();
        let mut cur = self.data;

        loop {
            if cur.is_null() { break; }

            let cur_o = unsafe { &mut *cur };

            if cur_o.color() == 2 {
                cur_o.set_color(0);
                prev = cur;
                cur = cur_o.next;
            } else {
                let unreached = cur;
                cur = cur_o.next;
                if prev.is_null() {
                    self.data = cur;
                } else {
                    unsafe { &mut *prev }.set_next(cur);
                }

                unsafe { libc::free(unreached as *mut libc::c_void) };
            }

        }
    }
}
