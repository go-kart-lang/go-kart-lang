use crate::{BinOp, Term, UnOp};

#[derive(Debug)]
pub struct Predef<'a, 'b> {
    pub items: Vec<(&'static str, crate::Exp)>,
    pub body: &'b Term<'a>,
}

impl<'a, 'b> Predef<'a, 'b> {
    pub fn new(body: &'b Term<'a>) -> Self {
        let items = Vec::from([
            ("print", un_op(UnOp::Print)),
            ("read", un_op(UnOp::Read)),
            ("i2s", un_op(UnOp::Int2Str)),
            ("s2i", un_op(UnOp::Str2Int)),
            ("d2s", un_op(UnOp::Double2Str)),
            ("s2d", un_op(UnOp::Str2Double)),
            ("d2i", un_op(UnOp::Double2Int)),
            ("i2d", un_op(UnOp::Int2Double)),
            ("vectorIntZeros", un_op(UnOp::VectorIntZeros)),
            ("vectorIntGet", bin_op(BinOp::VectorIntGet)),
            ("vectorIntUpdate", bin_op(BinOp::VectorIntUpdate)),
        ]);

        Self { items, body }
    }
}

fn un_op(un_op: UnOp) -> crate::Exp {
    crate::Exp::Abs(
        crate::Pat::Var(0),
        crate::Exp::Sys1(un_op, crate::Exp::Var(0).ptr()).ptr(),
    )
}

fn bin_op(bin_op: BinOp) -> crate::Exp {
    crate::Exp::Abs(
        crate::Pat::Var(0),
        crate::Exp::Abs(
            crate::Pat::Var(1),
            crate::Exp::Sys2(bin_op, crate::Exp::Var(0).ptr(), crate::Exp::Var(1).ptr()).ptr(),
        )
        .ptr(),
    )
}
