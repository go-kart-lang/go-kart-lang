use crate::prim_op::PrimOp;

pub enum OpCode {
    Acc(u32),
    QuoteInt(i32),
    Push,
    Swap,
    Prim(PrimOp),
    Cur(u32),
    Return,
    App,
    Pack(u32),
    Skip,
    Stop,
    Call(u32),
    Gotofalse(u32),
    Switch(u32, u32),
    Goto(u32),
}
