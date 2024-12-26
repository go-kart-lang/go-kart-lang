use derive_new::new;

#[derive(Debug, new)]
pub struct Ast<'a> {
    pub defs: NeSeq<Def<'a>>,
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
    pub cons: Pipe<Con<'a>>,
}

#[derive(Debug, new)]
pub struct FuncDef<'a> {
    pub name: Ident<'a>,
    pub params: Seq<Ident<'a>>,
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
    pub params: Seq<Udent<'a>>,
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
    pub left: AppExpr<'a>,
    pub name: OprName<'a>,
    pub right: InfixExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct App<'a> {
    pub head: AtExprPtr<'a>,
    pub children: NeSeq<AtExprPtr<'a>>,
}

#[derive(Debug, new)]
pub struct Cond<'a> {
    pub cond: ExprPtr<'a>,
    pub left: ExprPtr<'a>,
    pub right: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct Abs<'a> {
    pub args: NeSeq<Ident<'a>>,
    pub body: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct Case<'a> {
    pub cond: ExprPtr<'a>,
    pub branches: NeSeq<Branch<'a>>,
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
pub struct LetFuncDef<'a> {
    pub name: Ident<'a>,
    pub params: Seq<Ident<'a>>,
    pub body: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub struct Let<'a> {
    pub kind: LetKind,
    pub funcs: NeSeq<LetFuncDef<'a>>,
    pub expr: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub enum AtExpr<'a> {
    Lit(Lit<'a>),
    Var(Var<'a>),
    WrapExpr(WrapExpr<'a>),
}

#[derive(Debug, new)]
pub struct WrapExpr<'a> {
    pub expr: ExprPtr<'a>,
}

#[derive(Debug, new)]
pub enum AppExpr<'a> {
    App(App<'a>),
    AtExpr(AtExpr<'a>),
}

#[derive(Debug, new)]
pub enum InfixExpr<'a> {
    AppExpr(AppExpr<'a>),
    Abs(Abs<'a>),
    Opr(Opr<'a>),
}

#[derive(Debug, new)]
pub enum Expr<'a> {
    InfixExpr(InfixExpr<'a>),
    Cond(Cond<'a>),
    Case(Case<'a>),
    Let(Let<'a>),
}

#[derive(Debug, new)]
pub struct As<'a> {
    pub name: Ident<'a>,
    pub pat: AtPatPtr<'a>,
}

#[derive(Debug, new)]
pub enum AtPat<'a> {
    As(As<'a>),
    Ident(Ident<'a>),
    Lit(Lit<'a>),
    WrapPat(WrapPat<'a>),
}

#[derive(Debug, new)]
pub struct PatCon<'a> {
    pub name: Udent<'a>,
    pub pats: Seq<AtPatPtr<'a>>, // TODO: maybe NeSeq?
}

#[derive(Debug, new)]
pub struct WrapPat<'a> {
    pub pat: PatPtr<'a>,
}

#[derive(Debug, new)]
pub enum Pat<'a> {
    AtPat(AtPat<'a>),
    PatCon(PatCon<'a>),
}

#[derive(Debug, new)]
pub struct Seq<T> {
    pub items: Vec<T>,
}

#[derive(Debug, new)]
pub struct NeSeq<T> {
    pub items: Vec<T>,
}

#[derive(Debug, new)]
pub struct Pipe<T> {
    pub items: Vec<T>,
}

pub type ExprPtr<'a> = Box<Expr<'a>>;
pub type AtExprPtr<'a> = Box<AtExpr<'a>>;
pub type InfixExprPtr<'a> = Box<InfixExpr<'a>>;

pub type PatPtr<'a> = Box<Pat<'a>>;
pub type AtPatPtr<'a> = Box<AtPat<'a>>;
