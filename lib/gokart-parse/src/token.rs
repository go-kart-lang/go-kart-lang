use gokart_core::Span;
use strum_macros::AsRefStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Span<'a>,
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
    Read,
    Print,
}

impl TokenKind {
    pub fn at<'a>(self, s: Span<'a>) -> Token<'a> {
        Token {
            kind: self,
            span: s,
        }
    }
}
