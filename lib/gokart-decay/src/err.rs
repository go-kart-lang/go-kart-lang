use miette::{Diagnostic, SourceSpan as Span};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum DecayErr {
    #[error("Unknown name <{1}>")]
    #[diagnostic()]
    UnknownName(#[label("here")] Span, String),
}

pub type DecayRes<'a, T> = Result<T, DecayErr>;
