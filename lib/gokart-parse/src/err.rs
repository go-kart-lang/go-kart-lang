use crate::token::TokenKind;
use gokart_core::{Loc, LocExt};
use miette::{Diagnostic, SourceSpan as Span};
use nom::{
    error::{ErrorKind, ParseError},
    IResult,
};
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ParseErr {
    #[error("Unexpected char <{1}>")]
    #[diagnostic()]
    UnexpectedChar(#[label("here")] Span, char),

    #[error("Parse error: {1:?}")]
    #[diagnostic()]
    NomError(#[label("here")] Span, ErrorKind),

    #[error("Bad int literal: {1}")]
    #[diagnostic()]
    BadIntLiteral(#[label("here")] Span, ParseIntError),

    #[error("Bad double literal: {1}")]
    #[diagnostic()]
    BadDoubleLiteral(#[label("here")] Span, ParseFloatError),

    #[error("Expect {} but got {}", ._1.as_ref(), ._2.as_ref())]
    #[diagnostic()]
    UnexpectedToken(#[label("here")] Span, TokenKind, TokenKind),
}

impl ParseErr {
    #[inline]
    pub fn err<'a, T>(self) -> ParseRes<'a, T> {
        Err(nom::Err::Error(self))
    }

    #[inline]
    pub fn failure<'a, T>(self) -> ParseRes<'a, T> {
        Err(nom::Err::Failure(self))
    }

    #[inline]
    pub fn span(&self) -> &Span {
        use ParseErr::*;

        match self {
            UnexpectedChar(span, _) => span,
            NomError(span, _) => span,
            BadIntLiteral(span, _) => span,
            BadDoubleLiteral(span, _) => span,
            UnexpectedToken(span, _, _) => span,
        }
    }
    #[inline]
    pub fn begin(&self) -> usize {
        self.span().offset()
    }
}

impl<'a> ParseError<Loc<'a>> for ParseErr {
    #[inline]
    fn from_error_kind(i: Loc<'a>, kind: ErrorKind) -> Self {
        ParseErr::NomError(i.into_span(), kind)
    }

    #[inline]
    fn append(_i: Loc<'a>, _kind: ErrorKind, other: Self) -> Self {
        other
    }

    #[inline]
    fn from_char(i: Loc<'a>, c: char) -> Self {
        ParseErr::UnexpectedChar(i.into_span(), c)
    }
}

pub type ParseRes<'a, T> = IResult<Loc<'a>, T, ParseErr>;
