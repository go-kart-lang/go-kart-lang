use crate::{
    state::State,
    value::{gvalue_cast, Ref},
};
use gokart_core::{BinOp, Double, GOpCode, Int, Label, NullOp, OpCode, Str, Tag, UnOp};
use rand::Rng;
use std::{
    io::{self, Write},
    iter,
};

pub trait Ops {
    fn execute(&self, state: &mut State);
}

impl Ops for NullOp {
    fn execute(&self, state: &mut State) {
        use NullOp::*;
        let ptr = match self {
            IntLit(val) => state.heap.allocate_int(*val),
            DoubleLit(val) => state.heap.allocate_double(*val),
            StrLit(val) => state.heap.allocate_str(val.clone()),
        };
        state.env = ptr;
        state.ip += 1;
    }
}

impl Ops for UnOp {
    fn execute(&self, state: &mut State) {
        use UnOp::*;

        match self {
            Print => {
                let val = gvalue_cast::<Str>(state.env);
                println!("{val}");
                state.env = std::ptr::null_mut();
            }
            Read => {
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                state.env = state.heap.allocate_str(input.trim().into());
            }
            Int2Str => {
                let val = gvalue_cast::<Int>(state.env);
                state.env = state.heap.allocate_str(val.to_string());
            }
            Str2Int => {
                let val = gvalue_cast::<Str>(state.env);
                let res = match val.parse::<i64>() {
                    Ok(x) => x,
                    Err(e) => panic!("Error at Str('{val}') to Int conversion: {e}"),
                };
                state.env = state.heap.allocate_int(res);
            }
            Double2Str => {
                let val = gvalue_cast::<Double>(state.env);
                state.env = state.heap.allocate_str(val.to_string());
            }
            Str2Double => {
                let val = gvalue_cast::<Str>(state.env);
                let res = match val.parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => panic!("Error at Str('{val}') to Double conversion: {e}"),
                };
                state.env = state.heap.allocate_double(res);
            }
            Double2Int => {
                let val = gvalue_cast::<Double>(state.env);
                state.env = state.heap.allocate_int(*val as Int);
            }
            Int2Double => {
                let val = gvalue_cast::<Int>(state.env);
                state.env = state.heap.allocate_double(*val as Double);
            }
            VectorIntLength => {
                let val = gvalue_cast::<rpds::Vector<Int>>(state.env);
                state.env = state.heap.allocate_int(val.len() as Int);
            }
            VectorIntFillRandom => {
                let size = *gvalue_cast::<Int>(state.env);
                let mut vec = rpds::Vector::new();
                let mut rng = rand::thread_rng();
                vec.extend((0..size).map(|_| rng.gen::<i64>()));

                state.env = state.heap.allocate_vector_int(vec);
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

        let val = match self {
            IntPlus => {
                state.env = state
                    .heap
                    .allocate_int(*gvalue_cast::<Int>(a_ref) + *gvalue_cast::<Int>(b_ref));
            }
            IntMul => {
                state.env = state
                    .heap
                    .allocate_int(*gvalue_cast::<Int>(a_ref) * *gvalue_cast::<Int>(b_ref));
            }
            IntMinus => {
                state.env = state
                    .heap
                    .allocate_int(*gvalue_cast::<Int>(a_ref) - *gvalue_cast::<Int>(b_ref));
            }
            IntDiv => {
                state.env = state
                    .heap
                    .allocate_int(*gvalue_cast::<Int>(a_ref) / *gvalue_cast::<Int>(b_ref));
            }
            IntLt => {
                state.env = state
                    .heap
                    .allocate_int((*gvalue_cast::<Int>(a_ref) < *gvalue_cast::<Int>(b_ref)) as Int);
            }
            IntLe => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Int>(a_ref) <= *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntEq => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Int>(a_ref) == *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntNe => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Int>(a_ref) != *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            IntGt => {
                state.env = state
                    .heap
                    .allocate_int((*gvalue_cast::<Int>(a_ref) > *gvalue_cast::<Int>(b_ref)) as Int);
            }
            IntGe => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Int>(a_ref) >= *gvalue_cast::<Int>(b_ref)) as Int,
                );
            }
            DoublePlus => {
                state.env = state
                    .heap
                    .allocate_double(*gvalue_cast::<Double>(a_ref) + *gvalue_cast::<Double>(b_ref));
            }
            DoubleMul => {
                state.env = state
                    .heap
                    .allocate_double(*gvalue_cast::<Double>(a_ref) * *gvalue_cast::<Double>(b_ref));
            }
            DoubleMinus => {
                state.env = state
                    .heap
                    .allocate_double(*gvalue_cast::<Double>(a_ref) - *gvalue_cast::<Double>(b_ref));
            }
            DoubleDiv => {
                state.env = state
                    .heap
                    .allocate_double(*gvalue_cast::<Double>(a_ref) / *gvalue_cast::<Double>(b_ref));
            }
            DoubleLt => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Double>(a_ref) < *gvalue_cast::<Double>(b_ref)) as Int,
                );
            }
            DoubleLe => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Double>(a_ref) <= *gvalue_cast::<Double>(b_ref)) as Int,
                );
            }
            DoubleEq => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Double>(a_ref) == *gvalue_cast::<Double>(b_ref)) as Int,
                );
            }
            DoubleNe => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Double>(a_ref) != *gvalue_cast::<Double>(b_ref)) as Int,
                );
            }
            DoubleGt => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Double>(a_ref) > *gvalue_cast::<Double>(b_ref)) as Int,
                );
            }
            DoubleGe => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<Double>(a_ref) >= *gvalue_cast::<Double>(b_ref)) as Int,
                );
            }
            StrPlus => {
                let a_str = gvalue_cast::<String>(a_ref);
                let b_str = gvalue_cast::<String>(b_ref);
                state.env = state.heap.allocate_str(a_str.to_owned() + b_str);
            }
            StrEq => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<String>(a_ref) == *gvalue_cast::<String>(b_ref)) as Int,
                );
            }
            StrNe => {
                state.env = state.heap.allocate_int(
                    (*gvalue_cast::<String>(a_ref) != *gvalue_cast::<String>(b_ref)) as Int,
                );
            }
            VectorIntFill => {
                let size = *gvalue_cast::<Int>(a_ref);
                let val = *gvalue_cast::<Int>(b_ref);
                let mut vec = rpds::Vector::new();
                vec.extend(iter::repeat(val).take(size as usize));
                state.env = state.heap.allocate_vector_int(vec);
            }
            VectorIntGet => {
                let vec = gvalue_cast::<rpds::Vector<Int>>(a_ref);
                let idx = *gvalue_cast::<Int>(b_ref) as usize;
                let val = *vec.get(idx).unwrap();
                state.env = state.heap.allocate_int(val);
            }
            VectorIntUpdate => {
                let vec = gvalue_cast::<rpds::Vector<Int>>(a_ref);
                let (idx_ref, v_ref) = *gvalue_cast::<(Ref, Ref)>(b_ref);
                let idx = *gvalue_cast::<Int>(idx_ref) as usize;
                let v = *gvalue_cast::<Int>(v_ref);
                state.env = state.heap.allocate_vector_int(vec.set(idx, v).unwrap());
            }
            VectorIntUpdateMut => {
                let vec = gvalue_cast::<rpds::Vector<Int>>(a_ref);
                let (idx_ref, v_ref) = *gvalue_cast::<(Ref, Ref)>(b_ref);
                let idx = *gvalue_cast::<Int>(idx_ref) as usize;
                let v = *gvalue_cast::<Int>(v_ref);

                vec.set_mut(idx, v);
            }
        };

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
                    state.env = gvalue_cast::<(Ref, Ref)>(state.env).0
                }
                state.env = gvalue_cast::<(Ref, Ref)>(state.env).1;
                state.ip += 1;
            }
            Rest(n) => {
                for _ in 0..*n {
                    state.env = gvalue_cast::<(Ref, Ref)>(state.env).0
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
                state.env = state.heap.allocate_closure(state.env, *label);
                state.ip += 1;
            }
            Return => {
                let r = state.stack.pop();
                state.ip = *gvalue_cast::<Label>(r);
            }
            Clear => {
                state.env = std::ptr::null_mut();
                state.ip += 1;
            }
            Cons => {
                let a = state.stack.pop();
                let b = state.env;
                state.env = state.heap.allocate_pair(a, b);
                state.ip += 1;
            }
            App => {
                let b = state.stack.pop();
                let (a, label) = gvalue_cast::<(Ref, Label)>(state.env);
                state.env = state.heap.allocate_pair(*a, b);

                let r = state.heap.allocate_label(state.ip + 1);
                state.stack.push(r);
                state.ip = *label;
            }
            Pack(tag) => {
                state.env = state.heap.allocate_tagged(*tag, state.env);
                state.ip += 1;
            }
            Skip => {
                state.ip += 1;
            }
            Stop => {
                state.is_running = false;
            }
            Call(label) => {
                let r = state.heap.allocate_label(state.ip + 1);
                state.stack.push(r);
                state.ip = *label;
            }
            GotoFalse(label) => {
                let new_env = state.stack.pop();
                let b = gvalue_cast::<Int>(state.env);
                state.env = new_env;
                if *b == 0 {
                    state.ip = *label;
                } else {
                    state.ip += 1;
                }
            }
            Switch(tag, label) => {
                let (cur_tag, b) = gvalue_cast::<(Tag, Ref)>(state.env);
                if *cur_tag == *tag {
                    let a = state.stack.pop();
                    state.env = state.heap.allocate_pair(a, *b);
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
