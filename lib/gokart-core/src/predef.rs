use crate::{Term, UnOp};

#[derive(Debug)]
pub struct Predef<'a> {
    pub items: Vec<(&'static str, UnOp)>,
    pub body: Term<'a>,
}

impl<'a> Predef<'a> {
    pub fn new(body: Term<'a>) -> Self {
        let items = Vec::from([
            ("print", UnOp::Print),
            ("read", UnOp::Read),
            ("i2s", UnOp::Int2Str),
            ("s2i", UnOp::Str2Int),
            ("d2s", UnOp::Double2Str),
            ("s2d", UnOp::Str2Double),
            ("d2i", UnOp::Double2Int),
            ("i2d", UnOp::Int2Double),
        ]);

        Self { items, body }
    }
}
