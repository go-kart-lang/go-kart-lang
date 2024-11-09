use ordered_float::NotNan;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Token<S> {
    Identifier(S),
    UIdentifier(S),
    Operator(S),
    NatLiteral(u64),
    IntLiteral(i64),
    DoubleLiteral(NotNan<f64>),
    StringLiteral(S),
    Data,
    Pipe,
    Semicolon,
    Comma,
    Let,
    Letrec,
    In,
    Equals,
    If,
    Then,
    Else,
    Case,
    Of,
    Backslash,
    Arrow,
    As,
    LBrace,
    LBracket,
    LParen,
    RBrace,
    RBracket,
    RParen,
    Infixl,
    Infixr,
}

pub type BorrowedToken<'input> = Token<&'input str>;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum LexicalError {
    UnexpectedEndOfString,
    BadLiteral,
}
