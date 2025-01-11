pub type Int = i64;
pub type Double = f64;
pub type Str = String;

pub type Tag = usize;
pub type Label = usize;

#[derive(Debug)]
pub enum Hint {
    Int,
    Double,
    Str,
}
