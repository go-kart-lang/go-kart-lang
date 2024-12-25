use crate::{
    loc::{Loc, Pos},
    token::Token,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum LexErr {
    #[error("Unclosed quote")]
    UnclosedQuote(Pos),
    #[error("Invalid integer literal")]
    InvalidIntLit(Pos),
    #[error("Unexpected symbol <{0}>")]
    UnexpectedSymbol(char, Pos),
}

pub type LexResult<'a> = Result<(Token<'a>, Loc), LexErr>;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
pub enum ParseErr {
    #[error("Lexical error")]
    LexError(#[from] LexErr),
    #[error("Unexpected end of token stream")]
    UnexpectedEnd,
    #[error("Unexpected token {0}")]
    UnexpectedToken(&'static str, Pos),
    #[error("Invalid Infix")] // todo
    InvalidInfix(Pos),
}

pub type ParseResult<T> = Result<T, ParseErr>;
