use gokart_core::{Double, Int, Label, Str, Tag};

pub type Ref = u32;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Empty,
    Int(Int),
    Double(Double),
    Str(Str),
    Label(Label),
    Pair(Ref, Ref),
    Tagged(Tag, Ref),
    Closure(Ref, Label),
}

impl Value {
    pub fn as_int(&self) -> Int {
        match self {
            Value::Int(val) => *val,
            _ => panic!("Expected Value::Int"),
        }
    }

    pub fn as_double(&self) -> Double {
        match self {
            Value::Double(val) => *val,
            _ => panic!("Expected Value::Double"),
        }
    }

    pub fn as_str(&self) -> &Str {
        match self {
            Value::Str(val) => val,
            _ => panic!("Expected Value::Str"),
        }
    }

    pub fn as_label(&self) -> Label {
        match self {
            Value::Label(label) => *label,
            _ => panic!("Expected Value::Label"),
        }
    }

    pub fn as_pair(&self) -> (Ref, Ref) {
        match self {
            Value::Pair(left, right) => (*left, *right),
            _ => panic!("Expected Value::Pair"),
        }
    }

    pub fn as_tagged(&self) -> (Tag, Ref) {
        match self {
            Value::Tagged(tag, r) => (*tag, *r),
            _ => panic!("Expected Value::Tagged"),
        }
    }

    pub fn as_closure(&self) -> (Ref, Label) {
        match self {
            Value::Closure(r, label) => (*r, *label),
            _ => panic!("Expected Value::Closure"),
        }
    }
}
