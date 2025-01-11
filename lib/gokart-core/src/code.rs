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

impl FromIterator<OpCode> for Code {
    fn from_iter<T: IntoIterator<Item = OpCode>>(iter: T) -> Self {
        Code {
            data: Vec::from_iter(iter),
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
