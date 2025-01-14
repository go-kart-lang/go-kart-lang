use crate::ops::Ops;
use crate::value::{Value, ValueEnv};
use crate::{state::State, GC};
use gokart_core::OpCode;

pub struct VM {
    state: State,
    code: Vec<OpCode>,
    gc: GC,
}

impl VM {
    #[inline]
    pub fn new(code: Vec<OpCode>, gc: GC) -> Self {
        Self::from_state(State::new(), code, gc)
    }

    #[inline]
    pub fn from_state(state: State, code: Vec<OpCode>, gc: GC) -> Self {
        Self { state, code, gc }
    }

    #[inline]
    pub fn run(&mut self) {
        while self.state.is_running {
            self.code[self.state.ip].execute(&mut self.state);

            // if self.gc.is_necessary(&self.state) {
            //     self.gc.cleanup(&mut self.state);
            // }
        }

        self.gc.cleanup(&mut self.state);
    }

    pub fn cleanup(&mut self) {
        self.state.heap.clean();
    }

    // TODO: consider move this method to trait, because
    // it required only in tests
    pub fn cur_env(&self) -> *mut ValueEnv {
        self.state.env
    }

    pub fn cur_acc(&self) -> *mut Value {
        self.state.acc
    }
}

#[cfg(test)]
mod tests {
    use crate::value::ValueInt;

    use super::*;
    use gokart_core::{BinOp, GOpCode, NullOp};

    #[test]
    fn test() {
        let code = Vec::from([
            GOpCode::PushMark,
            GOpCode::Sys0(NullOp::IntLit(5)),
            GOpCode::Push,
            GOpCode::Cur(6),
            GOpCode::Apply,
            GOpCode::Stop,
            GOpCode::Sys0(NullOp::IntLit(4)), // lbl:6
            GOpCode::Push,
            GOpCode::Acc(0),
            GOpCode::Sys2(BinOp::IntPlus),
            GOpCode::Return
        ]);
        let gc = GC::new(10_000);
        let mut vm = VM::from_state(State::new(), code, gc);

        vm.run();

        let res = unsafe { &*(vm.cur_acc() as *mut ValueInt) };
        assert_eq!(res.data, 9);

        vm.cleanup();

        assert_eq!(vm.state.heap.data.is_null(), true);
    }

}
