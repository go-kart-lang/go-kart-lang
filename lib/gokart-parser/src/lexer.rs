use std::str::CharIndices;

use crate::token::{LexicalError, Token};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

fn is_ident_start(ch: char) -> bool {
    match ch {
        '_' | 'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

fn is_ident_continue(ch: char) -> bool {
    match ch {
        '0'..='9' | '\'' => true,
        ch => is_ident_start(ch),
    }
}

fn is_operator(c: char) -> bool {
    match c {
        '!' | '#' | '$' | '%' | '&' | '*' | '+' | '-' | '.' | '/' | '<' | '=' | '>' | '?' | '@'
        | '\\' | '^' | '|' | '~' | ':' => true,
        _ => false,
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    chars: std::iter::Peekable<CharIndices<'input>>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    fn slice(&self, start: usize, end: usize) -> &'input str {
        &self.input[start..end]
    }

    fn take_while(&mut self, cond: impl Fn(char) -> bool) -> usize {
        while let Some((cur, ch)) = self.chars.peek() {
            if cond(*ch) {
                self.chars.next();
            } else {
                return *cur;
            }
        }
        self.input.len()
    }

    fn string_literal(&mut self, start: usize) -> Spanned<Token<&'input str>, usize, LexicalError> {
        self.take_while(|x| x != '"');
        let end = self.chars.next().map(|(x, _)| x);

        match end {
            None => Err(LexicalError::UnexpectedEndOfString),
            Some(end) => Ok((start, Token::StringLiteral(self.slice(start + 1, end)), end)),
        }
    }

    fn numeric_literal(
        &mut self,
        start: usize,
        is_neg: bool,
    ) -> Spanned<Token<&'input str>, usize, LexicalError> {
        let end = self.take_while(|x| x.is_digit(10));

        let token = if is_neg {
            self.slice(start, end).parse().map(|n| Token::IntLiteral(n))
        } else {
            self.slice(start, end).parse().map(|n| Token::NatLiteral(n))
        };

        token
            .map(|t| (start, t, end))
            .map_err(|_| LexicalError::BadLiteral)
    }

    fn identifier(&mut self, start: usize) -> Spanned<Token<&'input str>, usize, LexicalError> {
        let end = self.take_while(|x| is_ident_continue(x));
        let tok = match self.slice(start, end) {
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
            s if s.starts_with(char::is_uppercase) => Token::UIdentifier(s),
            s => Token::Identifier(s),
        };

        Ok((start, tok, end))
    }

    fn operator(&mut self, start: usize) -> Spanned<Token<&'input str>, usize, LexicalError> {
        let end = self.take_while(|x| is_operator(x));

        let tok = match self.slice(start, end) {
            "=" => Token::Equals,
            "\\" => Token::Backslash,
            "|" => Token::Pipe,
            "->" => Token::Arrow,
            s => Token::Operator(s),
        };

        Ok((start, tok, end))
    }

    fn test_lookahead(&mut self, test: impl Fn(char) -> bool) -> bool {
        self.chars.peek().map_or(false, |(_, ch)| test(*ch))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token<&'input str>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch)) = self.chars.next() {
            return match ch {
                '{' => Some(Ok((start, Token::LBrace, start + 1))),
                '}' => Some(Ok((start, Token::RBrace, start + 1))),
                '(' => Some(Ok((start, Token::LParen, start + 1))),
                ')' => Some(Ok((start, Token::RParen, start + 1))),
                '[' => Some(Ok((start, Token::LBracket, start + 1))),
                ']' => Some(Ok((start, Token::RBracket, start + 1))),
                ',' => Some(Ok((start, Token::Comma, start + 1))),
                '"' => Some(self.string_literal(start)),
                ch if ch.is_digit(10) || (ch == '-' && self.test_lookahead(|x| x.is_digit(10))) => {
                    Some(self.numeric_literal(start, ch == '-'))
                }
                ch if is_ident_start(ch) => Some(self.identifier(start)),
                ch if is_operator(ch) => Some(self.operator(start)),
                ch if ch.is_whitespace() => continue,
                _ => todo!(),
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn it_parses_nat_literal() {
        let mut lexer = Lexer::new("123");

        assert_eq!(lexer.next(), Some(Ok((0, Token::NatLiteral(123), 3))));
    }

    #[test]
    fn it_parses_nat_literals() {
        let mut lexer = Lexer::new("123 89");

        assert_eq!(lexer.next(), Some(Ok((0, Token::NatLiteral(123), 3))));
        assert_eq!(lexer.next(), Some(Ok((4, Token::NatLiteral(89), 6))));
    }

    #[test]
    fn it_parses_neg_int_literal() {
        let mut lexer = Lexer::new("-123");

        assert_eq!(lexer.next(), Some(Ok((0, Token::IntLiteral(-123), 4))));
    }

    #[test]
    fn it_parses_string_literal() {
        let mut lexer = Lexer::new(r#""the string""#);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::StringLiteral("the string"), 11)))
        );
    }

    #[test]
    fn it_parses_string_literals() {
        let mut lexer = Lexer::new(r#""the string" "another test string""#);

        assert_eq!(
            lexer.next(),
            Some(Ok((0, Token::StringLiteral("the string"), 11)))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok((13, Token::StringLiteral("another test string"), 33)))
        );
    }

    #[test]
    fn it_parses_string_identifiers() {
        let mut lexer = Lexer::new(r#"let f Cons x in plus f p"#);

        assert_eq!(lexer.next(), Some(Ok((0, Token::Let, 3))));
        assert_eq!(lexer.next(), Some(Ok((4, Token::Identifier("f"), 5))));
        assert_eq!(lexer.next(), Some(Ok((6, Token::UIdentifier("Cons"), 10))));
        assert_eq!(lexer.next(), Some(Ok((11, Token::Identifier("x"), 12))));
        assert_eq!(lexer.next(), Some(Ok((13, Token::In, 15))));
        assert_eq!(lexer.next(), Some(Ok((16, Token::Identifier("plus"), 20))));
        assert_eq!(lexer.next(), Some(Ok((21, Token::Identifier("f"), 22))));
        assert_eq!(lexer.next(), Some(Ok((23, Token::Identifier("p"), 24))));
    }
}
