use crate::op_code::OpCode;
use crate::prim_op::PrimOp;
use crate::value::{Label, Value};
use gokart_gc::{Heap, HeapRef};

pub struct VM {
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
            OpCode::Rest(n) => {
                for _ in 0..(*n) {
                    self.env = if let Value::Pair(h, _) = self.heap[self.env] {
                        h
                    } else {
                        panic!("not pair");
                    }
                }
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
                            PrimOp::IntLe => (v2i < v1i) as i32,
                            PrimOp::IntEq => (v2i == v1i) as i32,
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
            OpCode::Clear => {
                self.env = self.heap.alloc(Value::EmptyTuple);
                self.ip += 1;
            }
            OpCode::Cons => {
                let v2 = self.stack.pop().unwrap();
                let v1 = self.env;
                self.env = self.heap.alloc(Value::Pair(v2, v1));
                self.ip += 1;
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
            OpCode::Skip => {
                self.ip += 1;
            }
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
                    if b == 0 {
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let mut vm = VM::default(
//             vec![
//                 OpCode::Push,
//                 OpCode::QuoteInt(4),
//                 OpCode::Swap,
//                 OpCode::Cur(6),
//                 OpCode::App,
//                 OpCode::Stop,
//                 OpCode::Push,
//                 OpCode::Acc(0),
//                 OpCode::Swap,
//                 OpCode::Acc(1),
//                 OpCode::Prim(PrimOp::IntPlus),
//                 OpCode::Return,
//             ],
//             |h| {
//                 let p1 = h.alloc(Value::EmptyTuple);
//                 let p2 = h.alloc(Value::Int(1));
//                 h.alloc(Value::Pair(p1, p2))
//             },
//         );
//         vm.run();
//         match vm.cur_env() {
//             Value::Int(x) => assert_eq!(x, 5),
//             _ => assert!(false, "expected Value::Int(5)"),
//         };
//     }

//     fn even_program(n: i32, expected: i32) {
//         let mut vm = VM::default(
//             vec![
//                 OpCode::Push,
//                 OpCode::QuoteInt(n),
//                 OpCode::Swap,
//                 OpCode::Rest(0),
//                 OpCode::Call(7),
//                 OpCode::App,
//                 OpCode::Stop,
//                 OpCode::Cur(9),
//                 OpCode::Return,
//                 OpCode::Push,
//                 OpCode::Push,
//                 OpCode::Acc(0),
//                 OpCode::Swap,
//                 OpCode::QuoteInt(0),
//                 OpCode::Prim(PrimOp::IntEq),
//                 OpCode::Gotofalse(18),
//                 OpCode::QuoteInt(1),
//                 OpCode::Goto(32),
//                 OpCode::Push,
//                 OpCode::QuoteInt(1),
//                 OpCode::Swap,
//                 OpCode::Push,
//                 OpCode::Push,
//                 OpCode::Acc(0),
//                 OpCode::Swap,
//                 OpCode::QuoteInt(1),
//                 OpCode::Prim(PrimOp::IntMinus),
//                 OpCode::Swap,
//                 OpCode::Rest(1),
//                 OpCode::Call(7),
//                 OpCode::App,
//                 OpCode::Prim(PrimOp::IntMinus),
//                 OpCode::Return,
//             ],
//             |h| h.alloc(Value::EmptyTuple),
//         );
//         vm.run();
//         match vm.cur_env() {
//             Value::Int(ret) => assert_eq!(
//                 ret, expected,
//                 "even({}) = {} (expected {})",
//                 n, ret, expected
//             ),
//             _ => assert!(false, "expected Value::Int"),
//         };
//     }

//     #[test]
//     fn it_works2() {
//         even_program(0, 1);
//         even_program(56, 1);
//         even_program(1, 0);
//         even_program(55, 0);
//     }
// }
