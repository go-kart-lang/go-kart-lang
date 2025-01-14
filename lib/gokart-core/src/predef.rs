use crate::{BinOp, Exp, Pat, Type, UnOp};

#[derive(Debug)]
pub struct Predef {
    pub items: Vec<(&'static str, Exp, Type)>,
}

impl Default for Predef {
    fn default() -> Self {
        let items = Vec::from([
            ("print", un_op(UnOp::Print), func(Type::str(), Type::unit())),
            ("read", un_op(UnOp::Read), func(Type::unit(), Type::str())),
            ("i2s", un_op(UnOp::Int2Str), func(Type::int(), Type::str())),
            ("s2i", un_op(UnOp::Str2Int), func(Type::str(), Type::int())),
            (
                "d2s",
                un_op(UnOp::Double2Str),
                func(Type::double(), Type::str()),
            ),
            (
                "s2d",
                un_op(UnOp::Str2Double),
                func(Type::str(), Type::double()),
            ),
            (
                "d2i",
                un_op(UnOp::Double2Int),
                func(Type::double(), Type::int()),
            ),
            (
                "i2d",
                un_op(UnOp::Int2Double),
                func(Type::int(), Type::double()),
            ),
            // ("vectorIntFill", bin_op(BinOp::VectorIntFill)),
            // ("vectorIntGet", bin_op(BinOp::VectorIntGet)),
            // ("vectorIntUpdate", bin_op(BinOp::VectorIntUpdate)),
            // ("vectorIntLength", un_op(UnOp::VectorIntLength)),
            // ("vectorIntUpdateMut", bin_op(BinOp::VectorIntUpdateMut)),
        ]);

        Self { items }
    }
}

fn un_op(un_op: UnOp) -> Exp {
    Exp::Abs(Pat::Var(0), Exp::Sys1(un_op, Exp::Var(0).ptr()).ptr())
}

fn bin_op(bin_op: BinOp) -> Exp {
    Exp::Abs(
        Pat::Var(0),
        Exp::Abs(
            Pat::Var(1),
            Exp::Sys2(bin_op, Exp::Var(0).ptr(), Exp::Var(1).ptr()).ptr(),
        )
        .ptr(),
    )
}

fn func(from: Type, into: Type) -> Type {
    Type::Func(from.ptr(), into.ptr())
}
