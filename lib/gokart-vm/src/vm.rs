use crate::jit::Optimization;
use crate::ops::Ops;
use gokart_core::OpCode;


const JIT_THREHSOLD: usize = 100;

pub struct VM {
    pub inner: *mut gokart_runtime::gokart_machine,
    pub(crate) code: Vec<OpCode>,
    code_counter: Vec<usize>,
    optimizations: Vec<Box<dyn Optimization>>,
}

impl VM {
    pub fn new(code: Vec<OpCode>, optimizations: Vec<Box<dyn Optimization>>) -> Self {
        let code_len = code.len();
        Self {
            code,
            inner: gokart_runtime::gokart_machine_init(),
            code_counter: vec![0; code_len],
            optimizations,
        }
    }

    #[inline]
    pub fn machine(&self) -> &mut gokart_runtime::gokart_machine {
        unsafe { &mut *self.inner }
    }

    #[inline]
    pub fn gc(&self) -> &mut gokart_runtime::gokart_gc {
        unsafe { &mut *(self.machine().gc) }
    }

    #[inline]
    pub fn ip(&self) -> usize {
        unsafe { &mut *self.inner }.ip as usize
    }

    #[inline]
    pub fn run(&mut self) {
        let ptr = self.inner;
        let m = unsafe { &mut *self.inner };
        while m.is_running == 1 {
            // Check for optimizations
            let mut optimized = false;

            if self.code_counter[m.ip as usize] >= JIT_THREHSOLD {
                for opt in &self.optimizations {
                    if opt.can_apply(self) {
                        // println!("Applying optimization for: {:?}", self.code[self.state.ip]);

                        let (optimized_code, skip) = opt.apply(self);

                        let old_ip = self.ip();

                        // Replace the current instruction with the optimized one
                        for (offset, op) in optimized_code.into_iter().enumerate() {
                            op.execute(ptr);
                            self.code[m.ip as usize + offset] = op;
                        }

                        // Move the instruction pointer ahead by the number of optimized instructions
                        m.ip = (old_ip + skip) as u64;
                        optimized = true;
                        break;
                    }
                }
            }

            if !optimized {
                self.code_counter[m.ip as usize] += 1;
                self.code[m.ip as usize].execute(ptr);
            }
        }
    }

    pub fn cleanup(&mut self) {
        gokart_runtime::gokart_machine_free(self.inner);
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

        let mut vm = VM::new(code, vec![]);
        vm.gc().bytes_threshold = 10_000;
        vm.gc().objects_threshold = 10_000;

        let num = gokart_runtime::gokart_allocate_int(vm.inner, 1);
        vm.machine().env = gokart_runtime::gokart_allocate_pair(vm.inner, vm.machine().env, num);

        vm.run();

        let result = *gokart_runtime::gvalue_cast::<Int>(vm.machine().env);

        assert_eq!(result, 5, "Expected 5");

        vm.cleanup();
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

        let mut vm = VM::new(code, vec![]);
        vm.gc().bytes_threshold = 10_000;
        vm.gc().objects_threshold = 10_000;

        vm.run();
        let res = *gokart_runtime::gvalue_cast::<Int>(unsafe { &mut *vm.inner }.env);

        assert_eq!(
            res, expected,
            "is_even({n}) = {res:?} (expected {expected})",
        );
    }

    #[test]
    fn it_can_check_if_number_is_even() {
        even_program(0, 1);
        even_program(56, 1);
        even_program(1, 0);
        even_program(55, 0);
    }
}
