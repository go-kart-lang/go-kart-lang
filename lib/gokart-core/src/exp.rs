use crate::{Double, Int, PrimOp, Str, Tag};

#[derive(Debug)]
pub enum Sys {
    IntLit(Int),
    DoubleLit(Double),
    StrLit(Str),
    PrimOp(ExpPtr, PrimOp, ExpPtr),
    Print,
    ReadInt,
    ReadDouble,
    ReadStr,
}

pub type Var = usize;

#[derive(Debug)]
pub enum Exp {
    Empty,
    Var(Var),
    Sys(Sys),
    Pair(ExpPtr, ExpPtr),
    Con(Tag, ExpPtr),
    App(ExpPtr, ExpPtr),
    Abs(Pat, ExpPtr),
    Cond(ExpPtr, ExpPtr, ExpPtr),
    Case(ExpPtr, Vec<(Tag, Pat, Exp)>),
    Let(Pat, ExpPtr, ExpPtr),
    Letrec(Pat, ExpPtr, ExpPtr),
}

impl Exp {
    pub fn ptr(self) -> ExpPtr {
        Box::new(self)
    }
}

#[derive(Debug)]
pub enum Pat {
    Empty,
    Var(Var),
    Pair(PatPtr, PatPtr),
    Layer(Var, PatPtr),
}

impl Pat {
    pub fn ptr(self) -> PatPtr {
        Box::new(self)
    }
}

pub type PatPtr = Box<Pat>;
pub type ExpPtr = Box<Exp>;
