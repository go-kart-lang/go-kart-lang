use gokart_ast::Definitions;
use lalrpop_util::{lalrpop_mod, ParseError};
pub mod lexer;
pub mod token;

pub enum Error<'a> {
    InvalidConstructorName(&'a str),
}

lalrpop_mod!(
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[allow(unused_parens)]
    grammar
);

pub fn parse_input<'input>(
    input: &'input str,
) -> Result<Definitions<'input>, ParseError<usize, token::Token<&str>, token::LexicalError>> {
    let lexer = lexer::Lexer::new(input);
    let mut errors = vec![];

    grammar::DefinitionsParser::new().parse(&mut errors, lexer)
}
