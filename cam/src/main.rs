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
        while let Some(id) = self.pending.borrow_mut().pop_first() {
            if marked.insert(id) {
                self.heap.data.get(&id).unwrap().trace(&self);
            }
        }
        self.heap.data.retain(|&id, _| marked.contains(&id));
    }
}

type Label = usize;

#[derive(Copy, Clone)]
enum Value {
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

enum OpCode {
    Acc(u32),
    QuoteInt(i32),
    Push,
    Swap,
    IntPlus,
    IntMinus,
    IntDiv,
    IntMul,
    Cur(u32),
    Return,
    App,
    Pack(u32),
    Skip,
    Stop,
    Call(u32),
    Gotofalse(u32),
    Switch(u32, u32),
    Goto(u32),
}

struct VM {
    code: Vec<OpCode>,
    stack: Vec<HeapRef<Value>>,
    env: HeapRef<Value>,
    heap: Heap,
    ip: usize,
    is_stopped: bool,
}

impl VM {
    pub fn new(code: Vec<OpCode>, initial_env: fn(&mut Heap) -> HeapRef<Value>) -> VM {
        let mut heap = Heap::default();
        let env = initial_env(&mut heap);
        VM {
            code: code,
            stack: Vec::new(),
            env: env,
            heap: heap,
            ip: 0,
            is_stopped: false,
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.is_stopped
    }

    pub fn cur_env(&self) -> Value {
        self.heap[self.env]
    }

    pub fn step(&mut self) -> () {
        if self.is_stopped {
            return;
        }

        let cur_code = self.code.get(self.ip).unwrap();
        match cur_code {
            OpCode::Acc(n) => {
                for _ in 0..(*n) {
                    self.env = if let Value::Pair(h, _) = self.heap[self.env] {
                        h
                    } else {
                        panic!("not pair");
                    }
                }
                self.env = if let Value::Pair(_, h) = self.heap[self.env] {
                    h
                } else {
                    panic!("not pair");
                };

                self.ip += 1;
            }
            OpCode::QuoteInt(k) => {
                self.env = self.heap.alloc(Value::Int(*k));
                self.ip += 1;
            }
            OpCode::Push => {
                self.stack.push(self.env);
                self.ip += 1;
            }
            OpCode::Swap => {
                let tmp = self.stack.pop().unwrap();
                self.stack.push(self.env);
                self.env = tmp;
                self.ip += 1;
            }
            OpCode::IntPlus => {
                let v2 = self.stack.pop().unwrap();
                let v1 = self.env;
                self.env =
                    if let (Value::Int(v2i), Value::Int(v1i)) = (self.heap[v2], self.heap[v1]) {
                        self.heap.alloc(Value::Int(v2i + v1i))
                    } else {
                        panic!("wrong arguments for IntPlus");
                    };
                self.ip += 1;
            }
            OpCode::IntMinus => {
                let v2 = self.stack.pop().unwrap();
                let v1 = self.env;
                self.env =
                    if let (Value::Int(v2i), Value::Int(v1i)) = (self.heap[v2], self.heap[v1]) {
                        self.heap.alloc(Value::Int(v2i - v1i))
                    } else {
                        panic!("wrong arguments for IntPlus");
                    };
                self.ip += 1;
            }
            OpCode::IntDiv => {
                let v2 = self.stack.pop().unwrap();
                let v1 = self.env;
                self.env =
                    if let (Value::Int(v2i), Value::Int(v1i)) = (self.heap[v2], self.heap[v1]) {
                        self.heap.alloc(Value::Int(v2i / v1i))
                    } else {
                        panic!("wrong arguments for IntPlus");
                    };
                self.ip += 1;
            }
            OpCode::IntMul => {
                let v2 = self.stack.pop().unwrap();
                let v1 = self.env;
                self.env =
                    if let (Value::Int(v2i), Value::Int(v1i)) = (self.heap[v2], self.heap[v1]) {
                        self.heap.alloc(Value::Int(v2i * v1i))
                    } else {
                        panic!("wrong arguments for IntPlus");
                    };
                self.ip += 1;
            }
            OpCode::Cur(label) => {
                self.env = self.heap.alloc(Value::Closure(self.env, *label as Label));
                self.ip += 1;
            }
            OpCode::Return => {
                if let Value::Int(label) = self.heap[self.stack.pop().unwrap()] {
                    self.ip = label as Label;
                } else {
                    panic!("wrong argument for Return")
                }
            }
            OpCode::App => {
                let v2 = self.stack.pop().unwrap();
                if let Value::Closure(v1, label) = self.heap[self.env] {
                    self.env = self.heap.alloc(Value::Pair(v1, v2));
                    self.stack
                        .push(self.heap.alloc(Value::Int((self.ip + 1) as i32)));
                    self.ip = label;
                } else {
                    panic!("wrong arguments for App")
                }
            }
            OpCode::Pack(c) => {
                self.env = self.heap.alloc(Value::Tagged(*c, self.env));
                self.ip += 1;
            }
            OpCode::Skip => (),
            OpCode::Stop => {
                self.is_stopped = true;
            }
            OpCode::Call(label) => {
                self.stack
                    .push(self.heap.alloc(Value::Int((self.ip + 1) as i32)));
                self.ip = *label as Label;
            }
            OpCode::Gotofalse(label) => {
                let new_env = self.stack.pop().unwrap();
                if let Value::Int(b) = self.heap[self.env] {
                    self.env = new_env;
                    if (b == 0) {
                        self.ip = *label as Label;
                    } else {
                        self.ip += 1;
                    }
                } else {
                    panic!("wrong argument for Gotofalse");
                }
            }
            OpCode::Switch(c, l) => {
                if let Value::Tagged(ci, v1) = self.heap[self.env] {
                    if ci == *c {
                        let v2 = self.stack.pop().unwrap();
                        self.env = self.heap.alloc(Value::Pair(v2, v1));
                        self.ip = *l as Label;
                    } else {
                        self.ip += 1;
                    }
                } else {
                    panic!("wrong argument for Switch");
                }
            }
            OpCode::Goto(l) => {
                self.ip = *l as Label;
            }
        }
    }
}

fn main() {
    let mut vm = VM::new(
        vec![
            OpCode::Push,
            OpCode::QuoteInt(4),
            OpCode::Swap,
            OpCode::Cur(6),
            OpCode::App,
            OpCode::Stop,
            OpCode::Push,
            OpCode::Acc(0),
            OpCode::Swap,
            OpCode::Acc(1),
            OpCode::IntPlus,
            OpCode::Return,
        ],
        |h| {
            let p1 = h.alloc(Value::EmptyTuple);
            let p2 = h.alloc(Value::Int(1));
            h.alloc(Value::Pair(p1, p2))
        },
    );

    while !vm.is_stopped() {
        vm.step();
        match vm.cur_env() {
            Value::EmptyTuple => {
                println!("empty tuple");
            }
            Value::Int(res) => {
                println!("int {}", res);
            }
            Value::Pair(heap_ref, heap_ref1) => {
                println!("pair");
            }
            Value::Tagged(_, heap_ref) => {
                println!("tagged");
            }
            Value::Closure(heap_ref, _) => {
                println!("closure");
            }
            Value::CClosure(l) => {
                println!("cclosure {}", l);
            }
        }
    }
}
