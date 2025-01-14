use crate::{Double, Int, Label, Offset, Str, Tag};

#[derive(Debug, Clone, PartialEq)]
pub enum NullOp {
    IntLit(Int),
    // DoubleLit(Double),
    // StrLit(Str),
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
    VectorIntLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    IntPlus,
    // IntMul,
    // IntMinus,
    // IntDiv,
    // IntLt,
    // IntLe,
    // IntEq,
    // IntNe,
    // IntGt,
    // IntGe,
    // DoublePlus,
    // DoubleMul,
    // DoubleMinus,
    // DoubleDiv,
    // DoubleLt,
    // DoubleLe,
    // DoubleEq,
    // DoubleNe,
    // DoubleGt,
    // DoubleGe,
    // StrPlus,
    // StrEq,
    // StrNe,
    // VectorIntFill,
    // VectorIntGet,
    // VectorIntUpdate,
    // VectorIntUpdateMut,
}

#[derive(Debug, PartialEq)]
pub enum GOpCode<L, O> {
    Acc(u32),
    Appterm,
    Apply,
    Push,
    PushMark,
    Grab,
    Sys0(NullOp),
    // Sys1(UnOp),
    Sys2(BinOp),
    Cur(L),
    Return,
    Let,
    Endlet(u32),
    Dummies(u32),
    Update(u32),
    Makeblock(u64, u32),
    Getfield(u32),
    Setfield(u32),
    Branch(O),
    Branchif(O),
    Branchifnot(O),
    Switch(Vec<O>),
    Stop,
}

pub type OpCode = GOpCode<Label, Offset>;
