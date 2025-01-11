use crate::{state::State, value::Value};
use gokart_core::{GOpCode, Int, OpCode, PrimOp};
use std::io;
use std::io::Write;

pub trait Ops {
    fn execute(&self, state: &mut State);
}

impl Ops for OpCode {
    #[inline]
    fn execute(&self, state: &mut State) {
        use GOpCode::*;

        match *self {
            Acc(n) => {
                for _ in 0..n {
                    state.env = state.cur_env().as_pair().0
                }
                state.env = state.cur_env().as_pair().1;
                state.ip += 1;
            }
            Rest(n) => {
                for _ in 0..n {
                    state.env = state.cur_env().as_pair().0
                }
                state.ip += 1;
            }
            Push => {
                state.stack.push(state.env);
                state.ip += 1;
            }
            Swap => {
                let tmp = state.stack.pop();
                state.stack.push(state.env);
                state.env = tmp;
                state.ip += 1;
            }
            Prim(op) => {
                // todo
                // QuoteInt(val) => {
                //     state.env = state.alloc(Value::Int(val));
                //     state.ip += 1;
                // }
                let a_ref = state.stack.pop();
                let b_ref = state.env;
                let a = state.heap[a_ref].as_int();
                let b = state.heap[b_ref].as_int();

                let res = match op {
                    PrimOp::IntPlus => a + b,
                    PrimOp::IntMul => a * b,
                    PrimOp::IntMinus => a - b,
                    PrimOp::IntDiv => a / b,
                    PrimOp::IntLe => Int::from(a < b),
                    // PrimOp::IntLeq => Int::from(a <= b),
                    PrimOp::IntEq => Int::from(a == b),
                    // PrimOp::IntNeq => Int::from(a != b),
                    // PrimOp::IntGe => Int::from(a > b),
                    // PrimOp::IntGeq => Int::from(a >= b),
                    PrimOp::Print => {
                        println!("GOKART OUTPUT: {}", b);
                        b
                    }
                };
                state.env = state.alloc(Value::Int(res));
                state.ip += 1;
            }
            Cur(label) => {
                let closure = Value::Closure(state.env, label);
                state.env = state.alloc(closure);
                state.ip += 1;
            }
            Return => {
                let r = state.stack.pop();
                state.ip = state.heap[r].as_label();
            }
            Clear => {
                state.env = state.alloc(Value::Empty);
                state.ip += 1;
            }
            Cons => {
                let a = state.stack.pop();
                let b = state.env;
                state.env = state.alloc(Value::Pair(a, b));
                state.ip += 1;
            }
            App => {
                let b = state.stack.pop();
                let (a, label) = state.cur_env().as_closure();
                state.env = state.alloc(Value::Pair(a, b));

                let r = state.alloc(Value::Label(state.ip + 1));
                state.stack.push(r);
                state.ip = label;
            }
            Pack(tag) => {
                state.env = state.alloc(Value::Tagged(tag, state.env));
                state.ip += 1;
            }
            Skip => {
                state.ip += 1;
            }
            Stop => {
                state.is_running = false;
            }
            Call(label) => {
                let r = state.alloc(Value::Label(state.ip + 1));
                state.stack.push(r);
                state.ip = label;
            }
            GotoFalse(label) => {
                let new_env = state.stack.pop();
                let b = state.cur_env().as_int();
                state.env = new_env;
                if b == 0 {
                    state.ip = label;
                } else {
                    state.ip += 1;
                }
            }
            Switch(tag, label) => {
                let (cur_tag, b) = state.cur_env().as_tagged();
                if cur_tag == tag {
                    let a = state.stack.pop();
                    state.env = state.alloc(Value::Pair(a, b));
                    state.ip = label;
                } else {
                    state.ip += 1;
                }
            }
            Goto(label) => {
                state.ip = label;
            }
            Read => {
                print!("GOKART INPUT: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let val: Int = input.trim_end().parse().unwrap();
                state.env = state.alloc(Value::Int(val));
                state.ip += 1;
            }
        }
    }
}
