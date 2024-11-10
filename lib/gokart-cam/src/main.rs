use gokart_parser::parse_input;

fn main() {
    let result = parse_input(
        r#"
        data IntList = Nil | Cons Int IntList
        data Option = None | Some Int

        infixl + 5

        let f = 5

        let headOption x = case x of
            | Nil -> None;
            | Cons x _ -> Some x;

        let fix f = letrec x = f x; in x
    "#,
    )
    .unwrap();
    for x in result.0 {
        println!("{:?}", x);
    }
}
