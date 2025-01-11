use crate::{
    err::ParseRes,
    token::{Token, TokenKind},
};
use gokart_core::Loc;
use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_until, take_while},
    character::complete::{char as chr, digit1, multispace0, satisfy},
    combinator::{map, opt, recognize},
    sequence::{pair, tuple},
};

fn single<'a>(val: &'static str, kind: TokenKind) -> impl Fn(Loc<'a>) -> ParseRes<Token<'a>> {
    move |i: Loc| map(tag(val), |s| kind.at(s))(i)
}

fn str_lit(i: Loc) -> ParseRes<Token> {
    let q = "\"";
    let res = recognize(tuple((tag(q), take_until(q), tag(q))));

    map(res, |s| TokenKind::Str.at(s))(i)
}

fn int_lit(i: Loc) -> ParseRes<Token> {
    let res = recognize(tuple((opt(chr('-')), digit1)));

    map(res, |s| TokenKind::Int.at(s))(i)
}

fn double_lit(i: Loc) -> ParseRes<Token> {
    let res = recognize(tuple((opt(chr('-')), digit1, chr('.'), digit1)));

    map(res, |s| TokenKind::Double.at(s))(i)
}

fn ident(i: Loc) -> ParseRes<Token> {
    let res = recognize(pair(
        satisfy(|c: char| c.is_ascii_alphabetic() || c == '_'),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
    ));

    map(res, |s: Loc| {
        let kind = match *s.fragment() {
            "let" => TokenKind::Let,
            "letrec" => TokenKind::Letrec,
            "data" => TokenKind::Data,
            "in" => TokenKind::In,
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "else" => TokenKind::Else,
            "case" => TokenKind::Case,
            "of" => TokenKind::Of,
            "infixl" => TokenKind::Infixl,
            "infixr" => TokenKind::Infixr,
            "as" => TokenKind::As,
            f if f.starts_with(|c: char| c.is_ascii_uppercase()) => TokenKind::Udent,
            _ => TokenKind::Ident,
        };

        kind.at(s)
    })(i)
}

fn opr(i: Loc) -> ParseRes<Token> {
    let res = is_a("!#$%&*+-./<=>?@\\^|~:");

    map(res, |s: Loc| {
        let kind = match *s.fragment() {
            "=" => TokenKind::Assign,
            "\\" => TokenKind::Backslash,
            "|" => TokenKind::Pipe,
            "->" => TokenKind::Arrow,
            _ => TokenKind::Opr,
        };

        kind.at(s)
    })(i)
}

pub fn token(i: Loc) -> ParseRes<Token> {
    let (i, _) = multispace0(i)?;

    alt((
        single("{", TokenKind::LBrace),
        single("}", TokenKind::RBrace),
        single("(", TokenKind::LParen),
        single(")", TokenKind::RParen),
        single("[", TokenKind::LBracket),
        single("]", TokenKind::RBracket),
        single(",", TokenKind::Comma),
        single(";", TokenKind::Semicolon),
        str_lit,
        double_lit,
        int_lit,
        ident,
        opr,
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gokart_core::LocExt;
    use nom::{combinator::eof, multi::many0, sequence::tuple};
    use TokenKind::*;

    type Tokens = Vec<(TokenKind, usize, usize)>;

    fn lex_tokens(i: Loc) -> ParseRes<Tokens> {
        let res = tuple((many0(token), multispace0, eof));

        map(res, |(tokens, _, _)| {
            tokens
                .into_iter()
                .map(|t| (t.kind, t.loc.begin(), t.loc.end()))
                .collect()
        })(i)
    }

    fn assert_tokens(input: &str, expected: Tokens) {
        let res = lex_tokens(Loc::new(input));

        match res {
            Ok((_, res)) => assert_eq!(expected, res),
            Err(e) => panic!("Unexpected error: {e:?}"),
        }
    }

    fn assert_lex_error(input: &str, begin: usize) {
        let res = lex_tokens(Loc::new(input));

        match res {
            Ok(res) => panic!("This test should fail, but got {res:?}"),
            Err(e) => match e {
                nom::Err::Error(e) => assert_eq!(begin, e.begin()),
                _ => panic!("Unexpected error: {e:?}"),
            },
        }
    }

    #[test]
    fn ok_int_literal() {
        assert_tokens("123", vec![(Int, 0, 3)]);
    }

    #[test]
    fn ok_int_literals() {
        assert_tokens("123  89", vec![(Int, 0, 3), (Int, 5, 7)]);
    }

    #[test]
    fn ok_negative_int_literal() {
        assert_tokens("-123", vec![(Int, 0, 4)]);
    }

    #[test]
    fn ok_double_literal() {
        assert_tokens("12.33", vec![(Double, 0, 5)]);
    }

    #[test]
    fn ok_negative_double_literal() {
        assert_tokens("-1.3", vec![(Double, 0, 4)]);
    }

    #[test]
    fn ok_string_literal() {
        assert_tokens(r#""the string""#, vec![(Str, 0, 12)]);
    }

    #[test]
    fn ok_string_literals() {
        assert_tokens(
            r#""the string" "and another  string""#,
            vec![(Str, 0, 12), (Str, 13, 34)],
        );
    }

    #[test]
    fn ok_identifiers() {
        assert_tokens(
            "let f Cons x in plus f p",
            vec![
                (Let, 0, 3),
                (Ident, 4, 5),
                (Udent, 6, 10),
                (Ident, 11, 12),
                (In, 13, 15),
                (Ident, 16, 20),
                (Ident, 21, 22),
                (Ident, 23, 24),
            ],
        );
    }

    #[test]
    fn ok_operators() {
        assert_tokens(
            " if 14 = x+3",
            vec![
                (If, 1, 3),
                (Int, 4, 6),
                (Assign, 7, 8),
                (Ident, 9, 10),
                (Opr, 10, 11),
                (Int, 11, 12),
            ],
        );
    }

    #[test]
    fn ok_braces() {
        assert_tokens(
            "( { x} ][ (-42))",
            vec![
                (LParen, 0, 1),
                (LBrace, 2, 3),
                (Ident, 4, 5),
                (RBrace, 5, 6),
                (RBracket, 7, 8),
                (LBracket, 8, 9),
                (LParen, 10, 11),
                (Int, 11, 14),
                (RParen, 14, 15),
                (RParen, 15, 16),
            ],
        );
    }

    #[test]
    fn err_unclosed_quote() {
        assert_lex_error(r#"letrec x = "some string"#, 11);
    }

    #[test]
    fn err_unexpected_symbol() {
        assert_lex_error("-> привет мир", 3);
    }
}
