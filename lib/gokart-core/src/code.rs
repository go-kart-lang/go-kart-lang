use crate::{Label, OpCode};
use std::ops;

pub struct Code {
    data: Vec<OpCode>,
}

impl<T> From<T> for Code
where
    T: IntoIterator<Item = OpCode>,
{
    #[inline]
    fn from(value: T) -> Self {
        Code {
            data: value.into_iter().collect(),
        }
    }
}

impl ops::Index<Label> for Code {
    type Output = OpCode;

    #[inline]
    fn index(&self, label: Label) -> &Self::Output {
        &self.data[label]
    }
}
