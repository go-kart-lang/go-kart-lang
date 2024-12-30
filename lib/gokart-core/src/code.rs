use std::ops;

use crate::{Label, OpCode};

pub struct Code {
    pub data: Vec<OpCode>,
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
        &self.data[usize::from(label)]
    }
}
