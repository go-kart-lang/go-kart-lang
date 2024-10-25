use gokart_gc::{HeapRef, Trace, Vacuum};

pub type Label = usize;

#[derive(Copy, Clone)]
pub enum Value {
    EmptyTuple,
    Int(i32),
    Pair(HeapRef<Value>, HeapRef<Value>),
    Tagged(u32, HeapRef<Value>),
    Closure(HeapRef<Value>, Label),
    CClosure(Label),
}

impl Trace for Value {
    fn trace<'a>(&self, vac: &Vacuum<'a>) {
        match &self {
            Value::EmptyTuple => (),
            Value::Int(_) => (),
            Value::Pair(heap_ref, heap_ref1) => {
                vac.mark(*heap_ref);
                vac.mark(*heap_ref1);
            }
            Value::Tagged(_, heap_ref) => vac.mark(*heap_ref),
            Value::Closure(heap_ref, _) => vac.mark(*heap_ref),
            Value::CClosure(_) => (),
        }
    }
}
