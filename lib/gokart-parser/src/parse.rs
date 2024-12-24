use std::iter::Peekable;

pub trait Parse {
    fn parse<T>(ts: T) -> Result<Self, ParseError>
    where
        T: Iterator<Item = Token>;
}

impl<'a> Parse for Ast<'a> {
    fn parse<T>(ts: Peekable<T>) -> Result<Self, ParseError>
    where
        T: Iterator<Item = Token>,
    {
        let mut defs = Vec::new();
        while let Some(_) = ts.peek() {
            defs.push(Def::parse(ts)?);
        }

        Ok(Ast { defs })
    }
}
