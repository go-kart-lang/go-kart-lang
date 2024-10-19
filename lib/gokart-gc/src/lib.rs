use std::any::{type_name, Any};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::marker::PhantomData;

pub trait AnyUpcast {
    fn as_any(&self) -> &(dyn 'static + Any);
    fn type_name(&self) -> &'static str;
}

impl<T: Any> AnyUpcast for T {
    fn as_any(&self) -> &(dyn 'static + Any) {
        self
    }
    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
}

pub trait Trace: AnyUpcast + 'static {
    /// Calls `vac.mark()` on any `HeapRef` reachable from `self`
    fn trace<'a>(&self, vac: &Vacuum<'a>);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct HeapRef<T> {
    id: usize,
    ty: PhantomData<T>,
}

#[derive(Default)]
pub struct Heap {
    data: HashMap<usize, Box<dyn Trace>>,
    next_id: usize,
}

impl<T: Any> std::ops::Index<HeapRef<T>> for Heap {
    type Output = T;
    fn index(&self, id: HeapRef<T>) -> &T {
        let any: &(dyn Trace) = &**self
            .data
            .get(&id.id)
            .expect(&format!("Unknown id {}", id.id));

        any.as_any().downcast_ref().expect(&format!(
            "Expected type {}, found type {}",
            type_name::<T>(),
            any.type_name()
        ))
    }
}

impl Heap {
    pub fn alloc<T: Trace>(&mut self, val: T) -> HeapRef<T> {
        let id = self.next_id;
        self.next_id += 1;
        self.data.insert(id, Box::new(val));
        HeapRef {
            id,
            ty: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn collect(&mut self) -> Vacuum<'_> {
        Vacuum {
            heap: self,
            pending: RefCell::new(BTreeSet::new()),
        }
    }
}

pub struct Vacuum<'a> {
    heap: &'a mut Heap,
    pending: RefCell<BTreeSet<usize>>,
}

impl<'a> Vacuum<'a> {
    pub fn mark<T>(&self, id: HeapRef<T>) {
        self.pending.borrow_mut().insert(id.id);
    }

    pub fn finish(self) {
        let mut marked = BTreeSet::new();
        loop {
            let tmp = self.pending.borrow_mut().pop_first();
            if let Some(id) = tmp {
                if marked.insert(id) {
                    self.heap.data.get(&id).unwrap().trace(&self);
                }
            } else {
                break;
            }
        }
        self.heap.data.retain(|&id, _| marked.contains(&id));
    }
}
