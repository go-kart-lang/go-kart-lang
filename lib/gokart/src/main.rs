mod pipeline;

use crate::pipeline::Pipeline;
use std::time::Instant;

const TASK_1_CONTENT: &str = include_str!("../res/task_1.gokart");
const TASK_2_CONTENT: &str = include_str!("../res/task_2.gokart");
const TASK_3_CONTENT: &str = include_str!("../res/task_3.gokart");

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

        infixl + 5

        letrec
            x = (22, 44, 66);
            y = (1, 2);
            lst = Cons (5, Cons (4, Nil ()));
        in lst
    "#;

    // let input = r#"
    //     letrec impl = \a b n ->
    //         if n == 0 then a
    //         else impl b (a + b) (n - 1);
    //     in letrec fib = \n -> impl 0 1 n;
    //     in fib 50
    // "#;

    let code_with_io = r#"
        letrec impl = \n res ->
            if n == 0 then res
            else impl (n - 1) (n * res);
        in letrec factorial = \n -> impl n 1;
        in print (factorial read)
    "#;

    let start = Instant::now();

    {
        let pipe = Pipeline::new(10_000);
        let res = pipe.run_from_string(code_with_io, false);
        println!("{:?}", res);
    }

    let elapsed = start.elapsed();
    println!("===============================");
    println!("Execution time: {:.3?}", elapsed);
}
