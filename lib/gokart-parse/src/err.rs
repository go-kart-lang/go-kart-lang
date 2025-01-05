use gokart_core::Span;
use nom::{
    error::{ErrorKind, ParseError},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct ParseErr<'a> {
    span: Span<'a>,
    msg: String,
}

impl<'a> ParseErr<'a> {
    pub fn new<S: Into<String>>(span: Span<'a>, msg: S) -> Self {
        Self {
            span,
            msg: msg.into(),
        }
    }

    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn line(&self) -> u32 {
        self.span.location_line()
    }

    pub fn offset(&self) -> usize {
        self.span.location_offset()
    }
}

impl<'a> ParseError<Span<'a>> for ParseErr<'a> {
    fn from_error_kind(i: Span<'a>, kind: ErrorKind) -> Self {
        Self::new(i, format!("Parse error {:?}", kind))
    }

    fn append(_i: Span<'a>, _kind: ErrorKind, other: Self) -> Self {
        other
    }

    fn from_char(i: Span<'a>, c: char) -> Self {
        Self::new(i, format!("Unexpected character <{c}>"))
    }
}

pub type ParseRes<'a, T> = IResult<Span<'a>, T, ParseErr<'a>>;
