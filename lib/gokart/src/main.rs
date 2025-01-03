use gokart_parse::decay;
use gokart_parse::parse;

fn main() {
    // let input = r#"
    //     data IntList = Nil | Cons Int IntList
    //     data Option = None | Some Int

    //     infixl + 5

    //     let f = 5

    //     let headOption x = case x of
    //         | Nil -> None;
    //         | Cons x _ -> Some x;

    //     let fix f = letrec x = f x; in x
    // "#;

    // impl b (a + b) (n - 1);

    let input = r#"
        let impl = \a b n -> 
            if n == 0 then a
            else impl b (a + b) (n - 1);
        in let fib = \n -> impl 0 0 n;
        in fib 50
    "#;
    let res = parse(input);

    println!("{:?}", res);

    let exp = decay(res.unwrap());

    println!("{:?}", exp);
}
