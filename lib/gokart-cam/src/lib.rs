use gokart_gc::{Heap, HeapRef, Trace, Vacuum};

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

enum PrimOp {
    IntPlus,
    IntMul,
    IntMinus,
    IntDiv,
}

enum OpCode {
    Acc(u32),
    QuoteInt(i32),
    Push,
    Swap,
    Prim(PrimOp),
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
    gc_threshold: usize,
}

impl VM {
    pub fn default(code: Vec<OpCode>, initial_env: fn(&mut Heap) -> HeapRef<Value>) -> VM {
        Self::new(code, initial_env, 10_000)
    }

    pub fn new(
        code: Vec<OpCode>,
        initial_env: fn(&mut Heap) -> HeapRef<Value>,
        gc_threshold: usize,
    ) -> VM {
        let mut heap = Heap::default();
        let env = initial_env(&mut heap);
        VM {
            code: code,
            stack: Vec::new(),
            env: env,
            heap: heap,
            ip: 0,
            is_stopped: false,
            gc_threshold,
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.is_stopped
    }

    pub fn cur_env(&self) -> Value {
        self.heap[self.env]
    }

    pub fn run(&mut self) -> () {
        while !self.is_stopped() {
            self.step();

            if self.heap.len() > self.gc_threshold {
                let vac = self.heap.collect();
                vac.mark(self.env);
                for e in &self.stack {
                    vac.mark(*e);
                }
                vac.finish();
            }
        }
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
            OpCode::Prim(prim_op) => {
                let v2 = self.stack.pop().unwrap();
                let v1 = self.env;
                self.env =
                    if let (Value::Int(v2i), Value::Int(v1i)) = (self.heap[v2], self.heap[v1]) {
                        let res = match prim_op {
                            PrimOp::IntPlus => v2i + v1i,
                            PrimOp::IntMul => v2i * v1i,
                            PrimOp::IntMinus => v2i - v1i,
                            PrimOp::IntDiv => v2i / v1i,
                        };

                        self.heap.alloc(Value::Int(res))
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
    let mut vm = VM::default(
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
            OpCode::Prim(PrimOp::IntPlus),
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
