use gokart_compile::Compiler;
use gokart_core::Code;
use gokart_decay::decay;
use gokart_parse::parse;
use gokart_vm::{State, Value, GC, VM};

fn main() {
    // let input = r#"
    //     data IntList = Nil | Cons Int IntList
    //     data Option = None | Some Int

    //     infixl + 5

    //     let
    //         headOption = \x -> case x of
    //             | Nil () -> None;
    //             | Cons (x, _) -> Some x;
    //         ;
    //     in headOption Nil
    // "#;

    let input = r#"
        letrec impl = \a b n ->
            if n == 0 then a
            else impl b (a + b) (n - 1);
        in letrec fib = \n -> impl 0 1 n;
        in fib 20
    "#;

    println!("{}", input);

    let ast = parse(input);
    // println!("{:?}", ast);

    let exp = decay(ast.unwrap());
    // println!("{:?}", exp);

    let code = Compiler::compile(&exp.unwrap());
    // println!("{:?}", code);

    let state = State::init_with(|h| h.alloc(Value::Empty));
    let gc = GC::new(10_000);
    let mut vm = VM::new(state, Code::from(code), gc);

    vm.run();

    let res = vm.cur_env();
    println!("{:?}", res);
}
