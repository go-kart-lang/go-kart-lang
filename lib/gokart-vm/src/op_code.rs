use crate::prim_op::PrimOp;

#[derive(Debug, PartialEq)]
pub enum GOpCode<Lbl> {
    Acc(u32),
    Rest(u32),
    QuoteInt(i32),
    Push,
    Swap,
    Prim(PrimOp),
    Cur(Lbl),
    Return,
    Clear,
    Cons,
    App,
    Pack(u32),
    Skip,
    Stop,
    Call(Lbl),
    Gotofalse(Lbl),
    Switch(u32, Lbl),
    Goto(Lbl),
}

pub type OpCode = GOpCode<u32>;
