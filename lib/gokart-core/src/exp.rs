use crate::{Double, Int, PrimOp, Str, Tag};

#[derive(Debug)]
pub enum Sys {
    IntLit(Int),
    DoubleLit(Double),
    StrLit(Str),
    Read,
    PrimOp(Exp, PrimOp, Exp),
}

pub type Var = usize;

#[derive(Debug)]
pub enum ExpNode {
    Var(Var),
    Sys(Sys),
    Empty,
    Pair(Exp, Exp),
    Con(Tag, Exp),
    App(Exp, Exp),
    Abs(Pat, Exp),
    Cond(Exp, Exp, Exp),
    Case(Exp, Vec<(Tag, Pat, Exp)>),
    Let(Pat, Exp, Exp),
    Letrec(Pat, Exp, Exp),
}

impl ExpNode {
    pub fn ptr(self) -> Exp {
        Box::new(self)
    }
}

#[derive(Debug)]
pub enum PatNode {
    Var(Var),
    Empty,
    Pair(Pat, Pat),
    Layer(Var, Pat),
}

impl PatNode {
    pub fn ptr(self) -> Pat {
        Box::new(self)
    }
}

pub type Pat = Box<PatNode>;
pub type Exp = Box<ExpNode>;
