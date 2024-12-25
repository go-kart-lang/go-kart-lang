use crate::{
    error::{ParseErr, ParseResult},
    lex::Lex,
    loc::Loc,
    token::Token,
};
use std::iter::Peekable;

pub type TokenStream<'a> = Peekable<Lex<'a>>;

pub trait TokenStremExt<'a> {
    fn nextf(&mut self) -> ParseResult<(Token<'a>, Loc)>;
    fn expect(&mut self, exp: Token<'a>) -> ParseResult<()>;
}

impl<'a> TokenStremExt<'a> for TokenStream<'a> {
    fn nextf(&mut self) -> ParseResult<(Token<'a>, Loc)> {
        match self.next() {
            None => Err(ParseErr::UnexpectedEnd),
            Some(r) => Ok(r?),
        }
    }

    fn expect(&mut self, exp: Token<'a>) -> ParseResult<()> {
        let (tok, loc) = self.nextf()?;

        match tok {
            _ if tok == exp => Ok(()),
            _ => Err(ParseErr::UnexpectedToken(tok.name(), loc.begin)),
        }
    }
}
