use gokart_core::Span;

#[derive(Debug, PartialEq)]
pub struct LogicErr<'a> {
    span: Span<'a>,
    msg: String,
}

impl<'a> LogicErr<'a> {
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

pub type LogicRes<'a, T> = Result<T, LogicErr<'a>>;
