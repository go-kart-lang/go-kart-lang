pub type Var = usize;
pub type Ctor = usize;

#[derive(Debug)]
pub enum Sys {} // todo

#[derive(Debug)]
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
