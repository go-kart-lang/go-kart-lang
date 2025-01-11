use crate::{Double, Int, Label, Str, Tag};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimOp {
    IntPlus,
    IntMul,
    IntMinus,
    IntDiv,
    IntLe,
    IntLeq,
    IntEq,
    IntNeq,
    IntGe,
    IntGeq,
    DoublePlus,
    DoubleMul,
    DoubleMinus,
    DoubleDiv,
    DoubleLe,
    DoubleLeq,
    DoubleEq,
    DoubleNeq,
    DoubleGe,
    DoubleGeq,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SysOp {
    IntLit(Int),
    DoubleLit(Double),
    StrLit(Str),
    Print,
    ReadInt,
    ReadDouble,
    ReadStr,
}

#[derive(Debug, PartialEq)]
pub enum GOpCode<L> {
    Acc(u32),
    Rest(u32),
    Push,
    Swap,
    SysOp(SysOp),
    PrimOp(PrimOp),
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
