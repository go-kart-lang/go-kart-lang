#[derive(Debug)]
pub struct Ast<'a> {
    pub defs: Vec<Def<'a>>,
}

#[derive(Debug)]
pub enum Def<'a> {
    Type(Type<'a>),
    Func(Func<'a>),
    Infix(Infix<'a>),
}

#[derive(Debug)]
pub struct Type<'a> {
    pub name: &'a str,
    pub cons: Vec<Con<'a>>,
}

#[derive(Debug)]
pub struct Func<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Expr<'a>,
}

#[derive(Debug)]
pub struct Infix<'a> {
    pub kind: InfixKind,
    pub name: &'a str,
    pub what: u64, // todo
}

#[derive(Debug)]
pub enum InfixKind {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Con<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Var<'a> {
    pub name: &'a str,
}

#[derive(Debug)]
pub enum Lit<'a> {
    Int(i64),
    Double(f64),  // todo
    Str(&'a str), // todo
}

#[derive(Debug)]
pub struct Opr<'a> {
    pub name: &'a str,
    pub left: Expr<'a>,
    pub right: Expr<'a>,
}

#[derive(Debug)]
pub struct App<'a> {
    pub head: Expr<'a>,
    pub children: Vec<Expr<'a>>,
}

#[derive(Debug)]
pub struct Cond<'a> {
    pub cond: Expr<'a>,
    pub left: Expr<'a>,
    pub right: Expr<'a>,
}

#[derive(Debug)]
pub struct Abs<'a> {
    pub args: Vec<&'a str>,
    pub body: Expr<'a>,
}

#[derive(Debug)]
pub struct Case<'a> {
    pub cond: Expr<'a>,
    pub branches: Vec<Branch<'a>>,
}

#[derive(Debug)]
pub struct Branch<'a> {
    pub pat: Pat<'a>,
    pub body: Expr<'a>,
}

#[derive(Debug)]
pub enum LetKind {
    NonRec,
    Rec,
}

#[derive(Debug)]
pub struct Let<'a> {
    pub kind: LetKind,
    pub funcs: Vec<Func<'a>>,
    pub expr: Expr<'a>,
}

#[derive(Debug)]
pub enum ExprNode<'a> {
    Var(Var<'a>),
    Lit(Lit<'a>),
    Opr(Opr<'a>),
    App(App<'a>),
    Cond(Cond<'a>),
    Abs(Abs<'a>),
    Case(Case<'a>),
    Let(Let<'a>),
}

#[derive(Debug)]
pub struct As<'a> {
    pub name: &'a str,
    pub pat: Pat<'a>,
}

#[derive(Debug)]
pub struct PatCon<'a> {
    pub name: &'a str,
    pub pats: Vec<Pat<'a>>,
}

#[derive(Debug)]
pub enum PatNode<'a> {
    Var(Var<'a>),
    Lit(Lit<'a>),
    As(As<'a>),
    Con(PatCon<'a>),
}

pub type Expr<'a> = Box<ExprNode<'a>>;
pub type Pat<'a> = Box<PatNode<'a>>;
