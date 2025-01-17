use crate::ops::Ops;
use crate::{state::State, GC};
use gokart_core::OpCode;
use crate::jit::Optimization;

pub struct VM {
    pub state: State,
    pub(crate) code: Vec<OpCode>,
    gc: GC,
    optimizations: Vec<Box<dyn Optimization>>,
}

impl VM {
    #[inline]
    pub fn new(code: Vec<OpCode>, gc: GC, optimizations: Vec<Box<dyn Optimization>>) -> Self {
        Self::from_state(State::new(), code, gc, optimizations)
    }

    #[inline]
    pub fn from_state(state: State, code: Vec<OpCode>, gc: GC, optimizations: Vec<Box<dyn Optimization>>) -> Self {
        Self { state, code, gc, optimizations }
    }

    #[inline]
    pub fn run(&mut self) {
        while self.state.is_running {
            // println!("Executing instruction at IP {}: {:?}", self.state.ip, self.code[self.state.ip]);

            // Check for optimizations
            let mut optimized = false;

            for opt in &self.optimizations {
                if opt.can_apply(self) {
                    // println!("Applying optimization for: {:?}", self.code[self.state.ip]);

                    let (optimized_code, skip) = opt.apply(self);

                    let old_ip = self.state.ip;

                    // Replace the current instruction with the optimized one
                    for (offset, op) in optimized_code.into_iter().enumerate() {
                        op.execute(&mut self.state);
                        self.code[self.state.ip + offset] = op;
                    }

                    // Move the instruction pointer ahead by the number of optimized instructions
                    self.state.ip = old_ip + skip;
                    optimized = true;
                    break;
                }
            }

            if !optimized {
                self.code[self.state.ip].execute(&mut self.state);
            }

            // Garbage collection check
            if self.gc.is_necessary(&self.state) {
                self.gc.cleanup(&mut self.state);
            }
        }
    }

    pub fn cleanup(&mut self) {
        self.state.env = std::ptr::null_mut();
        self.state.stack.clear();
        self.gc.cleanup(&mut self.state);
    }
}

#[cfg(test)]
mod tests {
    use crate::value::gvalue_cast;

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

        let mut state = State::new();
        let num = state.heap.allocate_int(1);
        state.env = state.heap.allocate_pair(state.env, num);

        let gc = GC::new(10_000, 10_000);
        let mut vm = VM::from_state(state, code, gc, vec![]);

        vm.run();

        let result = *gvalue_cast::<Int>(vm.state.env);

        assert_eq!(result, 5, "Expected 5");

        vm.cleanup();

        assert_eq!(vm.state.heap.bytes_allocated(), 0, "Expected empty heap");
        assert_eq!(vm.state.heap.objects_allocated(), 0, "Expected empty heap");
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
        let gc = GC::new(10_000, 10_000);
        let mut vm = VM::new(code, gc, vec![]);

        vm.run();
        let res = *gvalue_cast::<Int>(vm.state.env);

        assert_eq!(
            res, expected,
            "is_even({n}) = {res:?} (expected {expected})",
        );

        vm.cleanup();

        assert_eq!(vm.state.heap.bytes_allocated(), 0, "Expected empty heap");
        assert_eq!(vm.state.heap.objects_allocated(), 0, "Expected empty heap");
    }

    #[test]
    fn it_can_check_if_number_is_even() {
        even_program(0, 1);
        even_program(56, 1);
        even_program(1, 0);
        even_program(55, 0);
    }
}
