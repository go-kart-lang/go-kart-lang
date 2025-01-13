use crate::{Double, Int, Label, Str, Tag};

#[derive(Debug, Clone, PartialEq)]
pub enum NullOp {
    IntLit(Int),
    DoubleLit(Double),
    StrLit(Str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Print,
    Read,
    Int2Str,
    Str2Int,
    Double2Str,
    Str2Double,
    Double2Int,
    Int2Double,
    VectorIntZeros,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    IntPlus,
    IntMul,
    IntMinus,
    IntDiv,
    IntLt,
    IntLe,
    IntEq,
    IntNe,
    IntGt,
    IntGe,
    DoublePlus,
    DoubleMul,
    DoubleMinus,
    DoubleDiv,
    DoubleLt,
    DoubleLe,
    DoubleEq,
    DoubleNe,
    DoubleGt,
    DoubleGe,
    StrPlus,
    StrEq,
    StrNe,
    VectorIntGet,
    VectorIntUpdate,
}

#[derive(Debug, PartialEq)]
pub enum GOpCode<L> {
    Acc(u32),
    Rest(u32),
    Push,
    Swap,
    Sys0(NullOp),
    Sys1(UnOp),
    Sys2(BinOp),
    Cur(L),
    Return,
    Clear,
    Cons,
    App,
    Pack(Tag),
    Skip,
    Stop,
    Call(L),
    GotoFalse(L),
    Switch(Tag, L),
    Goto(L),
}

pub type OpCode = GOpCode<Label>;
