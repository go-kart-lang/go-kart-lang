use std::io::{self, Write};

use crate::{state::State, value::Value};
use gokart_core::{BinOp, GOpCode, Int, NullOp, OpCode, UnOp};

pub trait Ops {
    fn execute(&self, state: &mut State);
}

impl Ops for NullOp {
    fn execute(&self, state: &mut State) {
        use NullOp::*;
        let val = match self {
            IntLit(val) => Value::Int(*val),
            DoubleLit(val) => Value::Double(*val),
            StrLit(val) => Value::Str(val.clone()),
        };
        state.env = state.alloc(val);
        state.ip += 1;
    }
}

impl Ops for UnOp {
    fn execute(&self, state: &mut State) {
        use UnOp::*;

        match self {
            Print => {
                let val = state.cur_env().as_str();
                println!("GOKART OUTPUT: {val}");
                state.env = state.alloc(Value::Empty);
            }
            Read => {
                print!("GOKART INPUT: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                state.env = state.alloc(Value::Str(input.trim().into()));
            }
            Int2Str => {
                let val = state.cur_env().as_int();
                state.env = state.alloc(Value::Str(val.to_string()))
            }
            Str2Int => {
                let val = state.cur_env().as_str();
                let res = match val.parse::<i64>() {
                    Ok(x) => x,
                    Err(e) => panic!("Error at Str('{val}') to Int conversion: {e}"),
                };
                state.env = state.alloc(Value::Int(res));
            }
            Double2Str => {
                let val = state.cur_env().as_double();
                state.env = state.alloc(Value::Str(val.to_string()))
            }
            Str2Double => {
                let val = state.cur_env().as_str();
                let res = match val.parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => panic!("Error at Str('{val}') to Double conversion: {e}"),
                };
                state.env = state.alloc(Value::Double(res));
            }
            Double2Int => {
                let val = state.cur_env().as_double();
                state.env = state.alloc(Value::Int(val as i64))
            }
            Int2Double => {
                let val = state.cur_env().as_int();
                state.env = state.alloc(Value::Double(val as f64))
            }
        }
        state.ip += 1;
    }
}

impl Ops for BinOp {
    fn execute(&self, state: &mut State) {
        use BinOp::*;

        let a_ref = state.stack.pop();
        let b_ref = state.env;
        let a = &state.heap[a_ref];
        let b = &state.heap[b_ref];

        let val = match self {
            IntPlus => Value::Int(a.as_int() + b.as_int()),
            IntMul => Value::Int(a.as_int() * b.as_int()),
            IntMinus => Value::Int(a.as_int() - b.as_int()),
            // todo: zero division
            IntDiv => Value::Int(a.as_int() / b.as_int()),
            IntLt => Value::Int((a.as_int() < b.as_int()) as Int),
            IntLe => Value::Int((a.as_int() <= b.as_int()) as Int),
            IntEq => Value::Int((a.as_int() == b.as_int()) as Int),
            IntNe => Value::Int((a.as_int() != b.as_int()) as Int),
            IntGt => Value::Int((a.as_int() > b.as_int()) as Int),
            IntGe => Value::Int((a.as_int() >= b.as_int()) as Int),
            DoublePlus => Value::Double(a.as_double() + b.as_double()),
            DoubleMul => Value::Double(a.as_double() * b.as_double()),
            DoubleMinus => Value::Double(a.as_double() - b.as_double()),
            // todo: zero division
            DoubleDiv => Value::Double(a.as_double() / b.as_double()),
            DoubleLt => Value::Int((a.as_double() < b.as_double()) as Int),
            DoubleLe => Value::Int((a.as_double() <= b.as_double()) as Int),
            DoubleEq => Value::Int((a.as_double() == b.as_double()) as Int),
            DoubleNe => Value::Int((a.as_double() != b.as_double()) as Int),
            DoubleGt => Value::Int((a.as_double() > b.as_double()) as Int),
            DoubleGe => Value::Int((a.as_double() >= b.as_double()) as Int),
            StrPlus => Value::Str(a.as_str().to_owned() + b.as_str()),
            StrEq => Value::Int((a.as_str() == b.as_str()) as Int),
            StrNe => Value::Int((a.as_str() != b.as_str()) as Int),
        };

        state.env = state.alloc(val);
        state.ip += 1;
    }
}

impl Ops for OpCode {
    #[inline]
    fn execute(&self, state: &mut State) {
        use GOpCode::*;

        match self {
            Acc(n) => {
                for _ in 0..*n {
                    state.env = state.cur_env().as_pair().0
                }
                state.env = state.cur_env().as_pair().1;
                state.ip += 1;
            }
            Rest(n) => {
                for _ in 0..*n {
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
            Sys0(op) => op.execute(state),
            Sys1(op) => op.execute(state),
            Sys2(op) => op.execute(state),
            Cur(label) => {
                let closure = Value::Closure(state.env, *label);
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
                state.env = state.alloc(Value::Tagged(*tag, state.env));
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
                state.ip = *label;
            }
            GotoFalse(label) => {
                let new_env = state.stack.pop();
                let b = state.cur_env().as_int();
                state.env = new_env;
                if b == 0 {
                    state.ip = *label;
                } else {
                    state.ip += 1;
                }
            }
            Switch(tag, label) => {
                let (cur_tag, b) = state.cur_env().as_tagged();
                if cur_tag == *tag {
                    let a = state.stack.pop();
                    state.env = state.alloc(Value::Pair(a, b));
                    state.ip = *label;
                } else {
                    state.ip += 1;
                }
            }
            Goto(label) => {
                state.ip = *label;
            }
        }
    }
}
