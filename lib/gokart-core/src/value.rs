use crate::{Int, Label, Ref, Tag};

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
