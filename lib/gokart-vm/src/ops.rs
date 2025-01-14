use std::io::{self, Write};

use crate::{
    state::{StackValue, State},
    value::{Value, ValueBlock, ValueClosure, ValueEnv, ValueInt},
};
use gokart_core::{BinOp, GOpCode, Int, NullOp, OpCode, UnOp};

pub trait Ops {
    fn execute(&self, state: &mut State);
}

impl Ops for NullOp {
    fn execute(&self, state: &mut State) {
        let val = match self {
            NullOp::IntLit(i) => state.heap.allocate(ValueInt::new(*i)) as *mut Value,
        };
        state.acc = val;
        state.ip += 1;
    }
}

impl Ops for BinOp {
    fn execute(&self, state: &mut State) {
        let lhs = state.acc;
        let rhs = state.asp.pop().as_ptr();

        match self {
            BinOp::IntPlus => {
                let l = unsafe { &*(lhs as *mut ValueInt) }.data;
                let r = unsafe { &*(rhs as *mut ValueInt) }.data;

                state.acc = state.heap.allocate(ValueInt::new(l + r)) as *mut Value;
            }
        }

        state.ip += 1;
    }
}

impl Ops for OpCode {
    fn execute(&self, state: &mut State) {
        match self {
            GOpCode::Acc(n) => {
                state.acc = ValueEnv::access(state.env, *n as usize);
                state.ip += 1;
            }
            GOpCode::Appterm => {
                let closure = unsafe { &*(state.acc as *mut ValueClosure) };
                state.ip = closure.lbl;

                state.env = state.heap.allocate(ValueEnv::new(
                    state.asp.pop().as_ptr(),
                    closure.env as *mut ValueEnv,
                ));
            }
            GOpCode::Apply => {
                let closure = unsafe { &*(state.acc as *mut ValueClosure) };

                let h = state
                    .heap
                    .allocate(ValueClosure::new(state.ip + 1, state.env as *mut Value))
                    as *mut Value;
                state.rsp.push(StackValue::Ptr(h));

                state.ip = closure.lbl;
                state.env = state.heap.allocate(ValueEnv::new(
                    state.asp.pop().as_ptr(),
                    closure.env as *mut ValueEnv,
                ));
            }
            GOpCode::Push => {
                state.asp.push(StackValue::Ptr(state.acc));
                state.ip += 1;
            }
            GOpCode::PushMark => {
                state.asp.push(StackValue::Mark);
                state.ip += 1;
            }
            GOpCode::Cur(l) => {
                state.acc = state
                    .heap
                    .allocate(ValueClosure::new(*l, state.env as *mut Value))
                    as *mut Value;
                state.ip += 1;
            }
            GOpCode::Grab => match state.asp.pop() {
                StackValue::Ptr(v) => {
                    state.env = state.heap.allocate(ValueEnv::new(v, state.env));
                    state.ip += 1;
                }
                StackValue::Mark => {
                    state.acc = state
                        .heap
                        .allocate(ValueClosure::new(state.ip + 1, state.env as *mut Value))
                        as *mut Value;

                    let closure = unsafe { &*(state.rsp.pop().as_ptr() as *mut ValueClosure) };
                    state.env = closure.env as *mut ValueEnv;
                    state.ip = closure.lbl;
                }
            },
            GOpCode::Return => match state.asp.pop() {
                StackValue::Ptr(v) => {
                    let closure = unsafe { &*(state.acc as *mut ValueClosure) };

                    state.env = state
                        .heap
                        .allocate(ValueEnv::new(v, closure.env as *mut ValueEnv))
                        as *mut ValueEnv;
                        state.ip = closure.lbl;
                }
                StackValue::Mark => {
                    let closure = unsafe { &*(state.rsp.pop().as_ptr() as *mut ValueClosure) };
                    state.env = closure.env as *mut ValueEnv;
                    state.ip = closure.lbl;
                }
            },
            GOpCode::Let => {
                state.env = state
                    .heap
                    .allocate(ValueEnv::new(state.acc, state.env as *mut ValueEnv))
                    as *mut ValueEnv;
                state.ip += 1;
            }
            GOpCode::Endlet(cnt) => {
                for _ in 0..*cnt {
                    state.env = unsafe { &*state.env }.env;
                }
                state.ip += 1;
            }
            GOpCode::Dummies(cnt) => {
                for _ in 0..*cnt {
                    state.env = state.heap.allocate(ValueEnv::new(
                        std::ptr::null_mut(),
                        state.env as *mut ValueEnv,
                    )) as *mut ValueEnv;
                }
                state.ip += 1;
            }
            GOpCode::Update(n) => {
                let mut env = unsafe { &mut *state.env };
                for _ in 0..*n {
                    env = unsafe { &mut *env.env };
                }
                env.cur = state.acc;
                state.ip += 1;
            }
            GOpCode::Sys0(s) => s.execute(state),
            GOpCode::Sys2(s) => s.execute(state),
            GOpCode::Stop => {
                state.is_running = false;
            }
            GOpCode::Makeblock(tag, cnt) => {
                let mut data: Box<[*const Value]> =
                    std::iter::repeat_n(std::ptr::null(), *cnt as usize).collect();

                data[0] = state.acc;

                for i in 0..(*cnt - 1) {
                    data[(cnt - i - 1) as usize] = state.asp.pop().as_ptr()
                }

                state.acc = state.heap.allocate(ValueBlock::new(*tag, data)) as *mut Value;
                state.ip += 1;
            }
            GOpCode::Getfield(idx) => {
                let block = unsafe { &*(state.acc as *mut ValueBlock) };
                state.acc = block.data[*idx as usize] as *mut Value;
                state.ip += 1;
            }
            GOpCode::Setfield(idx) => {
                let block = unsafe { &mut *(state.acc as *mut ValueBlock) };
                block.data[*idx as usize] = state.asp.pop().as_ptr();
                state.ip += 1;
            }
            GOpCode::Branch(lbl) => {
                state.ip = (state.ip as isize + *lbl) as usize;
            }
            GOpCode::Branchif(lbl) => {
                let acc = unsafe { &*(state.acc as *mut ValueInt) };
                if acc.data != 0 {
                    state.ip = (state.ip as isize + *lbl) as usize;
                } else {
                    state.ip += 1;
                }
            }
            GOpCode::Branchifnot(lbl) => {
                let acc = unsafe { &*(state.acc as *mut ValueInt) };
                if acc.data == 0 {
                    state.ip = (state.ip as isize + *lbl) as usize;
                } else {
                    state.ip += 1;
                }
            },
            GOpCode::Switch(stmts) => {
                let acc = unsafe { &*state.acc };
                for (i, ofs) in stmts.iter().enumerate() {
                    if acc.tag() == i as u64 {
                        state.ip = (state.ip as isize + *ofs) as usize;
                        break;
                    }
                }
            }
        }
    }
}
