mod error;
mod lex;
mod loc;
mod parse;
mod token;
mod ts;

use error::ParseResult;
use gokart_core::ast::Ast;
use lex::Lex;
use parse::Parse;

pub fn parse<'a>(input: &'a str) -> ParseResult<Ast<'a>> {
    let lex = Lex::new(input);

    Ast::parse(&mut lex.peekable())
}
