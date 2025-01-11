use gokart_decay::decay;
use gokart_parse::parse;
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

    let input = r#"
        data IntList = Nil | Cons Int IntList
        data Option = None | Some Int

        letrec
            x = 22;
        in print (i2s x)
    "#;

    // let input = r#"
    //     letrec impl = \a b n ->
    //         if n == 0 then a
    //         else impl b (a + b) (n - 1);
    //     in letrec fib = \n -> impl 0 1 n;
    //     in fib 50
    // "#;

    // let code_with_io = r#"
    //     letrec impl = \n res ->
    //         if n == 0 then res
    //         else impl (n - 1) (n * res);
    //     in letrec factorial = \n -> impl n 1;
    //     in print (factorial read)
    // "#;

    //     let start = Instant::now();

    //     {
    //         let pipe = Pipeline::new(10_000);
    //         let res = pipe.run_from_string(code_with_io, false);
    //         println!("{:?}", res);
    //     }

    //     let elapsed = start.elapsed();
    //     println!("===============================");
    //     println!("Execution time: {:.3?}", elapsed);
    //

    //     let input = r#"
    // let f = \x -> x;
    // in f (f 2)
    //         "#;

    let res = parse(input).unwrap();
    let exp = decay(res);

    println!("{exp:?}");
}
