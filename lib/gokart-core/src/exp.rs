pub type Var = usize;
pub type Ctor = usize;

#[derive(Debug)]
pub enum Sys {} // todo

#[derive(Debug)]
pub struct ExpPair {
    pub left: Exp,
    pub right: Exp,
}

#[derive(Debug)]
pub struct Con {
    pub ctor: Ctor,
    pub exp: Exp,
}

#[derive(Debug)]
pub struct App {
    pub left: Exp,
    pub right: Exp,
}

#[derive(Debug)]
pub struct Abs {
    pub pat: Pat,
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Cond {
    pub cond: Exp,
    pub left: Exp,
    pub right: Exp,
}

#[derive(Debug)]
pub struct Case {
    pub cond: Exp,
    pub branches: Vec<Branch>,
}

#[derive(Debug)]
pub struct Branch {
    pub con: Con,
    pub pat: Pat,
    pub exp: Exp,
}

#[derive(Debug)]
pub struct Let {
    pub pat: Pat,
    pub exp: Exp,
    pub body: Exp,
}

#[derive(Debug)]
pub struct Letrec {
    pub pat: Pat,
    pub exp: Exp,
    pub body: Exp,
}

#[derive(Debug)]
pub struct PatPair {
    pub left: Pat,
    pub right: Pat,
}

#[derive(Debug)]
pub struct Layer {
    pub var: Var,
    pub pat: Pat,
}

#[derive(Debug)]
pub enum ExpNode {
    Var(Var),
    Sys(Sys),
    Empty,
    Pair(ExpPair),
    Con(Con),
    App(App),
    Abs(Abs),
    Cond(Cond),
    Case(Case),
    Let(Let),
    Letrec(Letrec),
}

#[derive(Debug)]
pub enum PatNode {
    Var(Var),
    Empty,
    Pair(PatPair),
    Layer(Layer),
}

pub type Pat = Box<PatNode>;
pub type Exp = Box<ExpNode>;
