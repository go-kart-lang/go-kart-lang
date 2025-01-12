use gokart_compile::compile;
use gokart_decay::decay;
use gokart_parse::parse;
use gokart_vm::{GC, VM};
// use miette::Error;

fn main() {
    // let input = r#"
    //     data IntList = Nil | Cons Int IntList
    //     data Option = None | Some Int

    //     infixl + 5

    //     letrec
    //         headOption = \x -> case x of
    //             | Nil () -> None;
    //             | Cons (x, _) -> Some x;
    //         ;
    //     in headOption Nil
    // "#;

    // let input = r#"
    //     data IntList = Nil | Cons Int IntList
    //     data Option = None | Some Int

    //     let
    //         x = 22;
    //     in print (i2s (x + 33))
    // "#;

    // let input = r#"
    //     letrec impl = \a b n ->
    //         if n == 0 then a
    //         else impl b (a + b) (n - 1);
    //     in letrec fib = \n -> impl 0 1 n;
    //     in fib 50
    // "#;

    let input = r#"
        letrec impl = \n res ->
            if n == 0 then res
            else impl (n - 1) (n * res);
        in let factorial = \n -> impl n 1;
        in let n = s2i (read ());
        in print (i2s (factorial n))
    "#;

    let res = parse(input).unwrap();
    let exp = decay(&res);
    // eprintln!("{exp:?}");
    let code = compile(&exp);
    // eprintln!("{:?}", code.iter().enumerate().collect::<Vec<_>>());
    let mut vm = VM::new(code, GC::default());
    vm.run();
    // println!("VM env: {:?}", vm.cur_env())
}
