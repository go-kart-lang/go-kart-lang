use gokart_parser::parse_input;

fn main() {
    let result = parse_input(r#"data IntList = Nil | Cons Int IntList"#);
    println!("{:?}", result);
}
