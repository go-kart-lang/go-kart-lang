use gokart_vm::{OpCode, PrimOp2, Value, VM};

fn main() {
    let mut vm = VM::default(
        vec![
            OpCode::Push,
            OpCode::QuoteInt(4),
            OpCode::Swap,
            OpCode::Cur(6),
            OpCode::App,
            OpCode::Stop,
            OpCode::Push,
            OpCode::Acc(0),
            OpCode::Swap,
            OpCode::Acc(1),
            OpCode::Prim(PrimOp2::IntPlus),
            OpCode::Return,
        ],
        |h| {
            let p1 = h.alloc(Value::EmptyTuple);
            let p2 = h.alloc(Value::Int(1));
            h.alloc(Value::Pair(p1, p2))
        },
    );

    while !vm.is_stopped() {
        vm.step();
        match vm.cur_env() {
            Value::EmptyTuple => {
                println!("empty tuple");
            }
            Value::Int(res) => {
                println!("int {}", res);
            }
            Value::Pair(_heap_ref, _heap_ref1) => {
                println!("pair");
            }
            Value::Tagged(_tag, _heap_ref) => {
                println!("tagged");
            }
            Value::Closure(_heap_ref, _label) => {
                println!("closure");
            }
            Value::CClosure(_label) => {
                println!("cclosure {}", _label);
            }
        }
    }
}
