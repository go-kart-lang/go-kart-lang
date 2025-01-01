mod ast;
mod decay;
mod err;
mod lex;
mod parse;
mod scope;
mod token;

pub use decay::decay;
pub use parse::parse;
