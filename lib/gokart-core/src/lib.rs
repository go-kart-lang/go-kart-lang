use core::panic;
use std::ops;

pub type Int = i32;
pub type Tag = u32;
pub type Ref = u32;
pub type Label = usize;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Value {
    Empty,
    Int(Int),
    Label(Label),
    Pair(Ref, Ref),
    Tagged(Tag, Ref),
    Closure(Ref, Label),
    CClosure(Label),
}

impl Value {
    pub fn as_pair(self) -> (Ref, Ref) {
        match self {
            Value::Pair(a, b) => (a, b),
            _ => panic!("Expected Value::Pair"),
        }
    }

    pub fn as_label(self) -> Label {
        match self {
            Value::Label(label) => label,
            _ => panic!("Expected Value::Label"),
        }
    }

    pub fn as_closure(self) -> (Ref, Label) {
        match self {
            Value::Closure(r, label) => (r, label),
            _ => panic!("Expected Value::Closure"),
        }
    }

    pub fn as_int(self) -> Int {
        match self {
            Value::Int(int) => int,
            _ => panic!("Expected Value::Int"),
        }
    }

    pub fn as_tagged(self) -> (Tag, Ref) {
        match self {
            Value::Tagged(tag, r) => (tag, r),
            _ => panic!("Expected Value::Tagged"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PrimOp {
    IntPlus,
    IntMul,
    IntMinus,
    IntDiv,
    IntLe,
    IntEq,
}

#[derive(Debug)]
pub enum OpCode {
    Acc(u32),
    Rest(u32),
    QuoteInt(Int),
    Push,
    Swap,
    Prim(PrimOp),
    Cur(Label),
    Return,
    Clear,
    Cons,
    App,
    Pack(Tag),
    Skip,
    Stop,
    Call(Label),
    GotoFalse(Label),
    Switch(Tag, Label),
    Goto(Label),
}

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
        &self.data[usize::from(label)]
    }
}
