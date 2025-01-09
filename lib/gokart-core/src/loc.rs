use miette::{SourceOffset, SourceSpan as Span};
use nom_locate::LocatedSpan;

pub type Loc<'a> = LocatedSpan<&'a str>;

pub trait LocExt<'a> {
    fn val(&self) -> &'a str;
    fn begin(&self) -> usize;
    fn end(&self) -> usize;
    fn len(&self) -> usize;
    fn as_span(self) -> Span;
}

impl<'a> LocExt<'a> for Loc<'a> {
    #[inline]
    fn val(&self) -> &'a str {
        self.fragment()
    }

    #[inline]
    fn begin(&self) -> usize {
        self.location_offset()
    }

    #[inline]
    fn end(&self) -> usize {
        self.begin() + self.len()
    }

    #[inline]
    fn len(&self) -> usize {
        self.val().len()
    }

    #[inline]
    fn as_span(self) -> Span {
        Span::new(SourceOffset::from(self.begin()), self.len())
    }
}
