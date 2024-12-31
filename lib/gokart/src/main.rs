use gokart_parse::parse;

fn main() {
    let input = r#"
        data IntList = Nil | Cons Int IntList
        data Option = None | Some Int

        infixl + 5

        let f = 5

        let headOption x = case x of
            | Nil -> None;
            | Cons x _ -> Some x;

        let fix f = letrec x = f x; in x
    "#;
    let res = parse(input);

    println!("{:?}", res);
}
