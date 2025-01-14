pub type Int = i64;
pub type Double = f64;
pub type Str = String;

pub type Tag = u64;
pub type Label = usize;
pub type Offset = isize;

#[derive(Debug)]
pub enum Hint {
    Int,
    Double,
    Str,
}
