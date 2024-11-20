use super::trace::Trace;
use super::vacuum::Vacuum;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Heap<V, R = usize> {
    data: HashMap<R, V>,
    next_id: R,
}

impl<V> Heap<V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            next_id: 0,
        }
    }

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
        Vacuum::new(self)
    }

    pub(super) fn trace_item(&self, id: usize, vacuum: &Vacuum) {
        if let Some(item) = self.data.get(&id) {
            item.trace(vacuum);
        }
    }

    pub(super) fn retain_marked(&mut self, marked: &std::collections::BTreeSet<usize>) {
        self.data.retain(|&id, _| marked.contains(&id));
    }
}

impl<T: 'static> std::ops::Index<HeapRef<T>> for Heap {
    type Output = T;

    fn index(&self, id: HeapRef<T>) -> &T {
        let any: &(dyn Trace) = &**self
            .data
            .get(&id.id)
            .expect(&format!("Unknown id {}", id.id));

        any.as_any().downcast_ref().expect(&format!(
            "Expected type {}, found type {}",
            std::any::type_name::<T>(),
            any.type_name()
        ))
    }
}
