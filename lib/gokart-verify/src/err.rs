use miette::{Diagnostic, SourceSpan as Span};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum VerifyErr {
    // todo
    #[error("Unknown name <{1}>")]
    #[diagnostic()]
    UnknownName(#[label("here")] Span, String),
}

pub type VerifyRes<'a, T> = Result<T, VerifyErr>;
