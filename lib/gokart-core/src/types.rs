use nom_locate::LocatedSpan;

pub type Int = i64;
pub type Double = f64;
pub type Str = String;

pub type Tag = usize;
pub type Label = usize;

pub type Span<'a> = LocatedSpan<&'a str>;
