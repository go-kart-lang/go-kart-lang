use crate::{
    error::{LexErr, LexResult},
    loc::{Loc, Pos},
    token::Token,
};
use std::{iter::Peekable, str::CharIndices};

#[derive(Clone)]
pub struct Lex<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Lex<'a> {
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    fn _take_while<P>(&mut self, pred: P, begin: usize) -> (&'a str, Loc)
    where
        P: Fn(char) -> bool,
    {
        let mut end = begin;
        while let Some(&(pos, ch)) = self.chars.peek() {
            if pred(ch) {
                self.chars.next();
                end = pos;
            } else {
                break;
            }
        }
        (&self.input[begin..=end], Loc::new(begin, end))
    }

    #[inline]
    fn str_lit(&mut self, begin: usize) -> LexResult<'a> {
        let (s, loc) = self._take_while(|c| c != '"', begin + 1);

        match self.chars.next() {
            None => Err(LexErr::UnclosedQuote(Pos::new(begin))),
            Some(_) => Ok((Token::Str(s), loc)),
        }
    }

    #[inline]
    fn minus(&mut self, begin: usize) -> LexResult<'a> {
        match self.chars.peek() {
            Some(&(_, ch)) if is_digit(ch) => self.num_lit(begin),
            _ => self.opr(begin),
        }
    }

    #[inline]
    fn num_lit(&mut self, begin: usize) -> LexResult<'a> {
        let (s, loc) = self._take_while(is_digit, begin);

        match s.parse() {
            Ok(x) => Ok((Token::Int(x), loc)),
            Err(_) => Err(LexErr::InvalidIntLit(Pos::new(begin))),
        }
    }

    #[inline]
    fn ident(&mut self, begin: usize) -> LexResult<'a> {
        let (s, loc) = self._take_while(is_ident, begin);

        let tok = match s {
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
        };
        Ok((tok, loc))
    }

    #[inline]
    fn opr(&mut self, begin: usize) -> LexResult<'a> {
        let (s, loc) = self._take_while(is_opr, begin);

        let tok = match s {
            "=" => Token::Assign,
            "\\" => Token::Backslash,
            "|" => Token::Pipe,
            "->" => Token::Arrow,
            s => Token::Opr(s),
        };
        Ok((tok, loc))
    }
}

impl<'a> Iterator for Lex<'a> {
    type Item = LexResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((pos, ch)) = self.chars.next() {
            return match ch {
                '{' => Some(Ok((Token::LBrace, Loc::new(pos, pos)))),
                '}' => Some(Ok((Token::RBrace, Loc::new(pos, pos)))),
                '(' => Some(Ok((Token::LParen, Loc::new(pos, pos)))),
                ')' => Some(Ok((Token::RParen, Loc::new(pos, pos)))),
                '[' => Some(Ok((Token::LBracket, Loc::new(pos, pos)))),
                ']' => Some(Ok((Token::RBracket, Loc::new(pos, pos)))),
                ',' => Some(Ok((Token::Comma, Loc::new(pos, pos)))),
                ';' => Some(Ok((Token::Semicolon, Loc::new(pos, pos)))),
                '"' => Some(self.str_lit(pos)),
                '-' => Some(self.minus(pos)),
                _ if is_digit(ch) => Some(self.num_lit(pos)),
                _ if is_ident_begin(ch) => Some(self.ident(pos)),
                _ if is_opr(ch) => Some(self.opr(pos)),
                _ if is_empty(ch) => continue,
                _ => Some(Err(LexErr::UnexpectedSymbol(ch, Pos::new(pos)))),
            };
        }
        None
    }
}

#[inline]
fn is_digit(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false,
    }
}

#[inline]
fn is_ident_begin(ch: char) -> bool {
    match ch {
        '_' | 'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

#[inline]
fn is_ident(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => is_ident_begin(ch),
    }
}

#[inline]
fn is_opr(ch: char) -> bool {
    match ch {
        '!' | '#' | '$' | '%' | '&' | '*' | '+' | '-' | '.' | '/' | '<' | '=' | '>' | '?' | '@'
        | '\\' | '^' | '|' | '~' | ':' => true,
        _ => false,
    }
}

#[inline]
fn is_empty(ch: char) -> bool {
    ch.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    macro_rules! assert_tokens {
        ( $input:expr, $expected:expr ) => {
            let lexer = Lex::new($input);

            let tokens = lexer.collect::<Result<Vec<(Token, Loc)>, LexErr>>();
            assert!(tokens.is_ok());
            assert_eq!($expected, tokens.unwrap());
        };
    }

    macro_rules! assert_lex_error {
        ( $input:expr, $err:expr ) => {
            let lexer = Lex::new($input);

            let tokens = lexer.collect::<Result<Vec<(Token, Loc)>, LexErr>>();
            assert!(tokens.is_err());
            assert_eq!($err, tokens.unwrap_err());
        };
    }

    #[test]
    fn it_parses_int_literal() {
        assert_tokens!("123", vec![(Int(123), Loc::new(0, 2))]);
    }

    #[test]
    fn it_parses_int_literals() {
        assert_tokens!(
            "123  89",
            vec![(Int(123), Loc::new(0, 2)), (Int(89), Loc::new(5, 6))]
        );
    }

    #[test]
    fn it_parses_negative_int_literal() {
        assert_tokens!("-123", vec![(Int(-123), Loc::new(0, 3))]);
    }

    #[test]
    fn it_parses_string_literal() {
        assert_tokens!(
            r#""the string""#,
            vec![(Str("the string"), Loc::new(1, 10))]
        );
    }

    #[test]
    fn it_parses_string_literals() {
        assert_tokens!(
            r#""the string" "and another  string""#,
            vec![
                (Str("the string"), Loc::new(1, 10)),
                (Str("and another  string"), Loc::new(14, 32))
            ]
        );
    }

    #[test]
    fn it_parses_identifiers() {
        assert_tokens!(
            "let f Cons x in plus f p",
            vec![
                (Let, Loc::new(0, 2)),
                (Ident("f"), Loc::new(4, 4)),
                (Udent("Cons"), Loc::new(6, 9)),
                (Ident("x"), Loc::new(11, 11)),
                (In, Loc::new(13, 14)),
                (Ident("plus"), Loc::new(16, 19)),
                (Ident("f"), Loc::new(21, 21)),
                (Ident("p"), Loc::new(23, 23)),
            ]
        );
    }

    #[test]
    fn it_parses_operators() {
        assert_tokens!(
            " if 14 = x+3",
            vec![
                (If, Loc::new(1, 2)),
                (Int(14), Loc::new(4, 5)),
                (Assign, Loc::new(7, 7)),
                (Ident("x"), Loc::new(9, 9)),
                (Opr("+"), Loc::new(10, 10)),
                (Int(3), Loc::new(11, 11)),
            ]
        );
    }

    #[test]
    fn it_parses_braces() {
        assert_tokens!(
            "( { x} ][ (-42))",
            vec![
                (LParen, Loc::new(0, 0)),
                (LBrace, Loc::new(2, 2)),
                (Ident("x"), Loc::new(4, 4)),
                (RBrace, Loc::new(5, 5)),
                (RBracket, Loc::new(7, 7)),
                (LBracket, Loc::new(8, 8)),
                (LParen, Loc::new(10, 10)),
                (Int(-42), Loc::new(11, 13)),
                (RParen, Loc::new(14, 14)),
                (RParen, Loc::new(15, 15)),
            ]
        );
    }

    #[test]
    fn it_handles_unclosed_quote_error() {
        assert_lex_error!(
            r#"letrec x = "some string"#,
            LexErr::UnclosedQuote(Pos::new(11))
        );
    }

    #[test]
    fn it_handles_invalid_int_error() {
        assert_lex_error!(
            "if x = 100000000000000000000",
            LexErr::InvalidIntLit(Pos::new(7))
        );
    }

    #[test]
    fn it_handles_unexpected_symbol_error() {
        assert_lex_error!("-> привет мир", LexErr::UnexpectedSymbol('п', Pos::new(3)));
    }
}
