use crate::ops::Ops;
use crate::value::Value;
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
        Self::from_state(State::default(), code, gc)
    }

    #[inline]
    pub fn from_state(state: State, code: Vec<OpCode>, gc: GC) -> Self {
        Self { state, code, gc }
    }

    #[inline]
    pub fn run(&mut self) {
        while self.state.is_running {
            self.code[self.state.ip].execute(&mut self.state);

            if self.gc.is_necessary(&self.state) {
                self.gc.cleanup(&mut self.state);
            }
        }
    }

    #[inline]
    pub fn cur_env(&self) -> &Value {
        self.state.cur_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gokart_core::{BinOp, GOpCode, Int, NullOp};
    use GOpCode::*;

    #[test]
    fn it_can_add_one_and_four() {
        let code = Vec::from([
            Push,
            Sys0(NullOp::IntLit(4)),
            Swap,
            Cur(6),
            App,
            Stop,
            Push,
            Acc(0),
            Swap,
            Acc(1),
            Sys2(BinOp::IntPlus),
            Return,
        ]);
        let state = State::init_with(|h| {
            let p1 = h.alloc(Value::Empty);
            let p2 = h.alloc(Value::Int(1));
            h.alloc(Value::Pair(p1, p2))
        });
        let gc = GC::new(10_000);
        let mut vm = VM::from_state(state, code, gc);

        vm.run();
        let res = vm.cur_env();

        assert_eq!(&Value::Int(5), res, "Expect Value::Int(5)");
    }

    fn even_program(n: Int, expected: Int) {
        let code = Vec::from([
            Push,
            Sys0(NullOp::IntLit(n)),
            Swap,
            Rest(0),
            Call(7),
            App,
            Stop,
            Cur(9),
            Return,
            Push,
            Push,
            Acc(0),
            Swap,
            Sys0(NullOp::IntLit(0)),
            Sys2(BinOp::IntEq),
            GotoFalse(18),
            Sys0(NullOp::IntLit(1)),
            Goto(32),
            Push,
            Sys0(NullOp::IntLit(1)),
            Swap,
            Push,
            Push,
            Acc(0),
            Swap,
            Sys0(NullOp::IntLit(1)),
            Sys2(BinOp::IntMinus),
            Swap,
            Rest(1),
            Call(7),
            App,
            Sys2(BinOp::IntMinus),
            Return,
        ]);
        let gc = GC::new(10_000);
        let mut vm = VM::new(code, gc);

        vm.run();
        let res = vm.cur_env();

        assert_eq!(
            &Value::Int(expected),
            res,
            "is_even({n}) = {res:?} (expected {expected})",
        )
    }

    #[test]
    fn it_can_check_if_number_is_even() {
        even_program(0, 1);
        even_program(56, 1);
        even_program(1, 0);
        even_program(55, 0);
    }
}
