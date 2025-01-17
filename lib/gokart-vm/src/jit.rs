use crate::VM;
use gokart_core::{BinOp, Int, NullOp, OpCode};

pub trait Optimization {
    fn can_apply(&self, vm: &VM) -> bool;

    fn apply(&self, vm: &VM) -> (Vec<OpCode>, usize);
}

pub struct TailCallOptimization;

impl Optimization for TailCallOptimization {
    fn can_apply(&self, vm: &VM) -> bool {
        let state = &vm.state;
        let code = &vm.code;

        let can_optimize = matches!(
            (code.get(state.ip), code.get(state.ip + 1)),
            (Some(OpCode::Call(_)), Some(OpCode::Return))
        );

        // println!("TCO can apply at IP {}: {}", state.ip, can_optimize);
        can_optimize
    }

    fn apply(&self, vm: &VM) -> (Vec<OpCode>, usize) {
        let code = &vm.code;
        if let OpCode::Call(label) = code[vm.state.ip] {
            // println!("TCO applied: replacing Call({:?}) + Return with Goto({:?})", label, label);
            (vec![OpCode::Goto(label)], 2)
        } else {
            (vec![], 0)
        }
    }
}

// ===================================================
pub struct DeadCodeElimination;

impl Optimization for DeadCodeElimination {
    fn can_apply(&self, vm: &VM) -> bool {
        matches!(
            vm.code.get(vm.state.ip),
            Some(OpCode::Call(_)) | Some(OpCode::Goto(_)) | Some(OpCode::GotoFalse(_))
        )
    }

    fn apply(&self, vm: &VM) -> (Vec<OpCode>, usize) {
        let ip = vm.state.ip;
        let code = &vm.code;

        let mut reachable = vec![false; code.len()];

        fn mark_reachable(code: &[OpCode], reachable: &mut [bool], start_ip: usize) {
            let mut stack = vec![start_ip];
            while let Some(ip) = stack.pop() {
                if ip >= code.len() || reachable[ip] {
                    continue;
                }
                reachable[ip] = true;

                match &code[ip] {
                    OpCode::Goto(label) => {
                        if *label < code.len() {
                            stack.push(*label);
                        }
                    }
                    OpCode::GotoFalse(label) => {
                        if *label < code.len() {
                            stack.push(*label);
                        }
                        stack.push(ip + 1);
                    }
                    OpCode::Call(label) => {
                        if *label < code.len() {
                            stack.push(*label);
                        }
                        stack.push(ip + 1); // Рекурсивные или другие вызовы должны следовать после
                    }
                    OpCode::Stop => {}
                    OpCode::Push | OpCode::Swap | OpCode::Return => {
                        stack.push(ip + 1);
                    }
                    _ => stack.push(ip + 1),
                }
            }
        }

        mark_reachable(code, &mut reachable, ip);

        let mut new_code = code.to_vec();

        for (i, &reachable) in reachable.iter().enumerate() {
            if !reachable {
                new_code[i] = OpCode::Skip; // Или другой заглушающий код
            }
        }

        let skipped = reachable.iter().filter(|&&r| !r).count();

        println!("DCE applied at IP {}: removed {} instructions", ip, skipped);

        (new_code, skipped)
    }
}

// ===============================================================
pub struct ConstantFolding;

impl Optimization for ConstantFolding {
    fn can_apply(&self, vm: &VM) -> bool {
        let state = &vm.state;
        let code = &vm.code;

        // Проверяем, если есть последовательность опкодов PUSH, INT_LIT, SWAP, INT_LIT, SYS2
        matches!(
            (
                code.get(state.ip),
                code.get(state.ip + 1),
                code.get(state.ip + 2),
                code.get(state.ip + 3),
                code.get(state.ip + 4)
            ),
            (
                Some(OpCode::Push),
                Some(OpCode::Sys0(NullOp::IntLit(_))),
                Some(OpCode::Swap),
                Some(OpCode::Sys0(NullOp::IntLit(_))),
                Some(OpCode::Sys2(_))
            )
        )
    }

    fn apply(&self, vm: &VM) -> (Vec<OpCode>, usize) {
        let code = &vm.code;
        let state = &vm.state;

        // Проверяем, если у нас последовательность: PUSH, INT_LIT, SWAP, INT_LIT, SYS2
        if let (
            Some(OpCode::Push),
            Some(OpCode::Sys0(NullOp::IntLit(left_value))),
            Some(OpCode::Swap),
            Some(OpCode::Sys0(NullOp::IntLit(right_value))),
            Some(OpCode::Sys2(op)),
        ) = (
            code.get(state.ip),
            code.get(state.ip + 1),
            code.get(state.ip + 2),
            code.get(state.ip + 3),
            code.get(state.ip + 4),
        ) {
            // Выполняем операцию с константами
            let result = match op {
                BinOp::IntPlus => left_value + right_value,
                BinOp::IntMul => left_value * right_value,
                BinOp::IntMinus => left_value - right_value,
                BinOp::IntDiv => left_value / right_value,
                BinOp::IntEq => (left_value == right_value) as i64,
                BinOp::IntGe => (left_value >= right_value) as i64,
                BinOp::IntGt => (left_value > right_value) as i64,
                BinOp::IntLe => (left_value <= right_value) as i64,
                BinOp::IntLt => (left_value < right_value) as i64,
                _ => panic!("Unsupported operation for constant folding!"),
            };

            // Возвращаем результат в виде одиночной операции с числовым значением
            println!("Applied optimization with result: {:?}", result);
            return (vec![OpCode::Sys0(NullOp::IntLit(result))], 5);
        }

        // Если не применили оптимизацию, то возвращаем пустое изменение
        (vec![], 0)
    }
}
