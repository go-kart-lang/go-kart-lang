use crate::{Pos, Token};
use std::{iter::Peekable, str::Chars};
use thiserror::Error;

pub struct Lex<'a> {
    input: &'a str,
    chars: Peekable<Chars<'a>>,
    pos: Pos,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexError {
    #[error("Unclosed quote at {0}")]
    UnclosedQuote(Pos),
    #[error("Invalid integer literal at {0}")]
    InvalidIntLit(Pos),
    #[error("Unexpected symbol {0} at {1}")]
    UnexpectedSymbol(char, Pos),
}

pub type LexResult<'a> = Result<Token<'a>, LexError>;

impl<'a> Lex<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().peekable(),
            pos: Pos::default(),
        }
    }

    fn _take_while<P>(&mut self, pred: P, begin: usize) -> &'a str
    where
        P: Fn(char) -> bool,
    {
        let mut end = begin;
        while let Some((pos, ch)) = self._peek() {
            if pred(ch) {
                self._next();
                end = pos;
            } else {
                break;
            }
        }
        &self.input[begin..=end]
    }

    fn _next(&mut self) -> Option<(Pos, char)> {
        let pos = self.pos;
        let res = self.chars.next();
        if let Some(ch) = res {
            self.pos.next(ch);
        }
        res.map(|ch| (pos, ch))
    }

    fn _peek(&mut self) -> Option<(usize, char)> {
        self.chars.peek().map(|&ch| (self.pos.idx, ch))
    }

    fn str_lit(&mut self, begin: Pos) -> LexResult<'a> {
        let s = self._take_while(|c| c != '"', begin.idx + 1);

        match self._next() {
            None => Err(LexError::UnclosedQuote(begin)),
            Some(_) => Ok(Token::Str(s)),
        }
    }

    fn minus(&mut self, begin: Pos) -> LexResult<'a> {
        match self._peek() {
            Some((_, ch)) if is_digit(ch) => self.num_lit(begin),
            _ => self.opr(begin),
        }
    }

    fn num_lit(&mut self, begin: Pos) -> LexResult<'a> {
        let s = self._take_while(is_digit, begin.idx);

        match s.parse() {
            Ok(x) => Ok(Token::Int(x)),
            Err(_) => Err(LexError::InvalidIntLit(begin)),
        }
    }

    fn ident(&mut self, begin: Pos) -> LexResult<'a> {
        let s = self._take_while(is_ident, begin.idx);

        Ok(match s {
            "let" => Token::Let,
            "letrec" => Token::Letrec,
            "data" => Token::Data,
            "in" => Token::In,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "case" => Token::Case,
            "of" => Token::Of,
            "infixl" => Token::Infixl,
            "infixr" => Token::Infixr,
            "as" => Token::As,
            s if s.starts_with(char::is_uppercase) => Token::Udent(s),
            s => Token::Ident(s),
        })
    }

    fn opr(&mut self, begin: Pos) -> LexResult<'a> {
        let s = self._take_while(is_opr, begin.idx);

        Ok(match s {
            "=" => Token::Eq,
            "\\" => Token::Backslash,
            "|" => Token::Pipe,
            "->" => Token::Arrow,
            s => Token::Opr(s),
        })
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = LexResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((pos, ch)) = self._next() {
            return match ch {
                '{' => Some(Ok(Token::LBrace)),
                '}' => Some(Ok(Token::LBrace)),
                '(' => Some(Ok(Token::LParen)),
                ')' => Some(Ok(Token::RParen)),
                '[' => Some(Ok(Token::LBracket)),
                ']' => Some(Ok(Token::RBracket)),
                ',' => Some(Ok(Token::Comma)),
                ';' => Some(Ok(Token::Semicolon)),
                '"' => Some(self.str_lit(pos)),
                '-' => Some(self.minus(pos)),
                _ if is_digit(ch) => Some(self.num_lit(pos)),
                _ if is_ident_begin(ch) => Some(self.ident(pos)),
                _ if is_opr(ch) => Some(self.opr(pos)),
                _ if is_empty(ch) => continue,
                _ => Some(Err(LexError::UnexpectedSymbol(ch, pos))),
            };
        }
        None
    }
}

fn is_digit(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_ident_begin(ch: char) -> bool {
    match ch {
        '_' | 'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

fn is_ident(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => is_ident_begin(ch),
    }
}

fn is_opr(ch: char) -> bool {
    match ch {
        '!' | '#' | '$' | '%' | '&' | '*' | '+' | '-' | '.' | '/' | '<' | '=' | '>' | '?' | '@'
        | '\\' | '^' | '|' | '~' | ':' => true,
        _ => false,
    }
}

fn is_empty(ch: char) -> bool {
    ch.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;
    use LexError::*;
    use Token::*;

    macro_rules! assert_tokens {
        ( $input:expr, $expected:expr ) => {
            let lexer = Lex::new($input);

            let tokens = lexer.collect::<Result<Vec<Token>, LexError>>();
            assert!(tokens.is_ok());
            assert_eq!($expected, tokens.unwrap());
        };
    }

    macro_rules! assert_lex_error {
        ( $input:expr, $err:expr ) => {
            let lexer = Lex::new($input);

            let tokens = lexer.collect::<Result<Vec<Token>, LexError>>();
            assert!(tokens.is_err());
            assert_eq!($err, tokens.unwrap_err());
        };
    }

    #[test]
    fn it_parses_int_literal() {
        assert_tokens!("123", vec![Int(123)]);
    }

    #[test]
    fn it_parses_int_literals() {
        assert_tokens!("123 89", vec![Int(123), Int(89)]);
    }

    #[test]
    fn it_parses_negative_int_literal() {
        assert_tokens!("-123", vec![Int(-123)]);
    }

    #[test]
    fn it_parses_string_literal() {
        assert_tokens!(r#""the string""#, vec![Str("the string")]);
    }

    #[test]
    fn it_parses_string_literals() {
        assert_tokens!(
            r#""the string" "and another  string""#,
            vec![Str("the string"), Str("and another  string")]
        );
    }

    #[test]
    fn it_parses_identifiers() {
        assert_tokens!(
            "let f Cons x in plus f p",
            vec![
                Let,
                Ident("f"),
                Udent("Cons"),
                Ident("x"),
                In,
                Ident("plus"),
                Ident("f"),
                Ident("p"),
            ]
        );
    }

    #[test]
    fn it_parses_operators() {
        assert_tokens!(
            " if 14 = x+3",
            vec![If, Int(14), Eq, Ident("x"), Opr("+"), Int(3),]
        );
    }

    #[test]
    fn it_handles_unclosed_quote_error() {
        assert_lex_error!(
            r#"letrec x = "some string"#,
            UnclosedQuote(Pos::new(1, 12, 11))
        );
    }

    #[test]
    fn it_handles_invalid_int_error() {
        assert_lex_error!(
            "if x = 100000000000000000000",
            InvalidIntLit(Pos::new(1, 8, 7))
        );
    }

    #[test]
    fn it_handles_unexpected_symbol_error() {
        assert_lex_error!("-> привет мир", UnexpectedSymbol('п', Pos::new(1, 4, 3)));
    }
}
