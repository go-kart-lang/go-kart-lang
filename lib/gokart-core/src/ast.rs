use derive_new::new;

#[derive(Debug, new)]
pub struct Ast<'a> {
    pub defs: Vec<Def<'a>>,
}

#[derive(Debug, new)]
pub enum Def<'a> {
    TypeDef(TypeDef<'a>),
    FuncDef(FuncDef<'a>),
    InfixDef(InfixDef<'a>),
}

#[derive(Debug, new)]
pub struct TypeDef<'a> {
    pub name: Udent<'a>,
    pub cons: Vec<Con<'a>>,
}

#[derive(Debug, new)]
pub struct FuncDef<'a> {
    pub name: Ident<'a>,
    pub params: Vec<Ident<'a>>,
    pub body: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct InfixDef<'a> {
    pub kind: InfixKind,
    pub name: OprName<'a>,
    pub priority: InfixPriority, // todo
}

#[derive(Debug, new)]
pub enum InfixKind {
    Left,
    Right,
}

#[derive(Debug, new)]
pub struct Con<'a> {
    pub name: Udent<'a>,
    pub params: Vec<Udent<'a>>,
}

#[derive(Debug, new)]
pub struct Var<'a> {
    pub name: &'a str,
}

#[derive(Debug, new)]
pub struct OprName<'a> {
    pub val: &'a str,
}

#[derive(Debug, new)]
pub struct InfixPriority {
    pub val: u64,
}

#[derive(Debug, new)]
pub struct Ident<'a> {
    pub val: &'a str,
}

#[derive(Debug, new)]
pub struct Udent<'a> {
    pub val: &'a str,
}

#[derive(Debug, new)]
pub enum Lit<'a> {
    Int(i64),
    Double(f64),
    Str(&'a str),
}

#[derive(Debug, new)]
pub struct Opr<'a> {
    pub name: OprName<'a>,
    pub left: ExprPtr<'a>,
    pub right: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct App<'a> {
    pub head: ExprPtr<'a>,
    pub children: Vec<ExprPtr<'a>>,
}

#[derive(Debug, new)]
pub struct Cond<'a> {
    pub cond: ExprPtr<'a>,
    pub left: ExprPtr<'a>,
    pub right: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct Abs<'a> {
    pub args: Vec<Ident<'a>>,
    pub body: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct Case<'a> {
    pub cond: ExprPtr<'a>,
    pub branches: Vec<Branch<'a>>,
}

#[derive(Debug, new)]
pub struct Branch<'a> {
    pub pat: PatPtr<'a>,
    pub body: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub enum LetKind {
    NonRec,
    Rec,
}

#[derive(Debug, new)]
pub struct Let<'a> {
    pub kind: LetKind,
    pub funcs: Vec<FuncDef<'a>>,
    pub expr: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub enum Expr<'a> {
    Var(Var<'a>),
    Lit(Lit<'a>),
    Opr(Opr<'a>),
    App(App<'a>),
    Cond(Cond<'a>),
    Abs(Abs<'a>),
    Case(Case<'a>),
    Let(Let<'a>),
}

// TODO: restore original structures
// make helper structures in parse.rs

#[derive(Debug, new)]
pub struct As<'a> {
    pub name: Ident<'a>,
    pub pat: AtPatPtr<'a>,
}

#[derive(Debug, new)]
pub enum AtPat<'a> {
    As(As<'a>),
    Var(Var<'a>),
    Lit(Lit<'a>),
    PatPtr(PatPtr<'a>),
}

#[derive(Debug, new)]
pub struct PatCon<'a> {
    pub name: Udent<'a>,
    pub pats: Vec<AtPatPtr<'a>>,
}

#[derive(Debug, new)]
pub enum Pat<'a> {
    AtPat(AtPat<'a>),
    PatCon(PatCon<'a>),
}

pub type ExprPtr<'a> = Box<Expr<'a>>;
pub type PatPtr<'a> = Box<Pat<'a>>;
pub type AtPatPtr<'a> = Box<AtPat<'a>>;
