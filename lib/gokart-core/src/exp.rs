use std::rc::Rc;

use crate::{Int, PrimOp};

pub type Var = usize;
pub type Ctor = usize;

#[derive(Debug, Clone)]
pub enum Sys {
    IntLit(Int),
    PrimOp(Exp, PrimOp, Exp),
} // TODO: StrLit, DoubleLit, etc

#[derive(Debug, Clone)]
pub enum ExpNode {
    Var(Var),
    Sys(Sys),
    Empty,
    Pair(Exp, Exp),
    Con(Ctor, Exp),
    App(Exp, Exp),
    Abs(Pat, Exp),
    Cond(Exp, Exp, Exp),
    Case(Exp, Vec<(Ctor, Pat, Exp)>),
    Let(Pat, Exp, Exp),
    Letrec(Pat, Exp, Exp),
}

impl ExpNode {
    pub fn ptr(self) -> Exp {
        Rc::new(self)
    }
}

#[derive(Debug, Clone)]
pub enum PatNode {
    Var(Var),
    Empty,
    Pair(Pat, Pat),
    Layer(Var, Pat),
}

impl PatNode {
    pub fn ptr(self) -> Pat {
        Rc::new(self)
    }
}

pub type Pat = Rc<PatNode>;
pub type Exp = Rc<ExpNode>;
