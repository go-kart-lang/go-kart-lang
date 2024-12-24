use crate::{Int, Label, PrimOp, Tag};

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
