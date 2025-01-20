use miette::{Diagnostic, SourceSpan as Span};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum VerifyErr {
    #[error("Unknown name: {1}")]
    #[diagnostic()]
    UnknownName(#[label("here")] Span, String),

    #[error("Unknown operation: {1}")]
    #[diagnostic()]
    UnknownOpr(#[label("here")] Span, String),

    #[error("Unknown type: {1}")]
    #[diagnostic()]
    UnknownType(#[label("here")] Span, String),

    #[error("Unknown constructor: {1}")]
    #[diagnostic()]
    UnknownCtor(#[label("here")] Span, String),

    #[error("Type already defined: {1}")]
    #[diagnostic()]
    TypeRedefinition(#[label("here")] Span, String),

    #[error("Ctor already defined: {1}")]
    #[diagnostic()]
    CtorRedefinition(#[label("here")] Span, String),

    #[error("Type mismatch: expected {1}, found {2}")]
    #[diagnostic()]
    TypeMismatch(#[label("here")] Span, String, String),

    #[error("Infinite type detected")]
    #[diagnostic()]
    InfiniteType(#[label("here")] Span),

    #[error("Type {1} doesn't match pattern {2}")]
    #[diagnostic()]
    PatternNotMatch(#[label("here")] Span, String, String),

    #[error("Invalid pattern: variable {1} already defined")]
    #[diagnostic()]
    PatternRedefinition(#[label("here")] Span, String),

    #[error("Invalid case: different branches have different types of constructors")]
    #[diagnostic()]
    InvalidBranchesType(#[label("here")] Span),

    #[error("Invalid case: branch for constructor already defined")]
    #[diagnostic()]
    BranchRedefinition(#[label("here")] Span),

    #[error("Invalid case: not all constructors are covered")]
    #[diagnostic()]
    BranchNotCovered(#[label("here")] Span),
}

pub type VerifyRes<T> = Result<T, VerifyErr>;
