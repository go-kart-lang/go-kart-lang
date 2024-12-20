use gokart_core::{Code, Int, OpCode, PrimOp, Value};
use gokart_vm::{State, GC, VM};
use OpCode::*;

fn even_program(n: Int, expected: Int) {
    let code = Code::from([
        Push,
        QuoteInt(n),
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
        QuoteInt(0),
        Prim(PrimOp::IntEq),
        GotoFalse(18),
        QuoteInt(1),
        Goto(32),
        Push,
        QuoteInt(1),
        Swap,
        Push,
        Push,
        Acc(0),
        Swap,
        QuoteInt(1),
        Prim(PrimOp::IntMinus),
        Swap,
        Rest(1),
        Call(7),
        App,
        Prim(PrimOp::IntMinus),
        Return,
    ]);
    let state = State::init_with(|h| h.alloc(Value::Empty));
    let gc = GC::new(10_000);
    let mut vm = VM::new(state, code, gc);

    vm.run();
    let res = *vm.cur_env();

    println!("Expected: {expected}");
    println!("Got: {res:?}");
}

fn main() {
    even_program(2, 0);

    // let result = parse_input(
    //     r#"
    //     data IntList = Nil | Cons Int IntList
    //     data Option = None | Some Int

    //     infixl + 5

    //     let f = 5

    //     let headOption x = case x of
    //         | Nil -> None;
    //         | Cons x _ -> Some x;

    //     let fix f = letrec x = f x; in x
    // "#,
    // )
    // .unwrap();
    // for x in result.0 {
    //     println!("{:?}", x);
    // }
}
