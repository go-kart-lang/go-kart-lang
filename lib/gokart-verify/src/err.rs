use miette::{Diagnostic, SourceSpan as Span};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum VerifyErr {
    // todo
    #[error("Unknown name <{1}>")]
    #[diagnostic()]
    UnknownName(#[label("here")] Span, String),

    #[error("Unknown type <{1}>")]
    #[diagnostic()]
    UnknownType(#[label("here")] Span, String),

    #[error("Unknown constructor <{1}>")]
    #[diagnostic()]
    UnknownCtor(#[label("here")] Span, String),

    #[error("Type {1} already defined")]
    #[diagnostic()]
    TypeRedefinition(#[label("here")] Span, String),

    #[error("Ctor {1} already defined")]
    #[diagnostic()]
    CtorRedefinition(#[label("here")] Span, String),

    #[error("Type mismatch: {1}")]
    #[diagnostic()]
    TypeMismatch(#[label("here")] Span, String),
}

pub type VerifyRes<T> = Result<T, VerifyErr>;
