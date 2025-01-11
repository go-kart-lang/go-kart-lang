use gokart_core::Loc;
use strum_macros::AsRefStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub loc: Loc<'a>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, AsRefStr)]
pub enum TokenKind {
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Str,
    Int,
    Double,
    Let,
    Letrec,
    Data,
    In,
    If,
    Then,
    Else,
    Case,
    Of,
    Infixl,
    Infixr,
    As,
    Udent,
    Ident,
    Assign,
    Backslash,
    Pipe,
    Arrow,
    Opr,
}

impl TokenKind {
    pub fn at(self, loc: Loc) -> Token {
        Token { kind: self, loc }
    }
}
