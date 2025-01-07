use crate::{Int, Label, PrimOp, Tag};

#[derive(Debug, PartialEq)]
pub enum GOpCode<L> {
    Acc(u32),
    Rest(u32),
    QuoteInt(Int), // todo (sys)
    Push,
    Swap,
    Prim(PrimOp),
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
    Read,
}

pub type OpCode = GOpCode<Label>;
