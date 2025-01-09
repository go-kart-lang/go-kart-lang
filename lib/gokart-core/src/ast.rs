use crate::Loc;
use derive_new::new;

#[derive(Debug, new)]
pub struct IntLit<'a> {
    pub val: i64,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct DoubleLit<'a> {
    pub val: f64,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct StrLit<'a> {
    pub val: &'a str,
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum Lit<'a> {
    Int(IntLit<'a>),
    Double(DoubleLit<'a>),
    Str(StrLit<'a>),
}

#[derive(Debug, new)]
pub struct Ast<'a> {
    pub defs: Vec<Def<'a>>,
    pub body: Term<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum Def<'a> {
    TypeDef(TypeDef<'a>),
}

#[derive(Debug, new)]
pub struct Name<'a> {
    pub val: &'a str,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Con<'a> {
    pub name: Name<'a>,
    pub params: Vec<Name<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct TypeDef<'a> {
    pub name: Name<'a>,
    pub cons: Vec<Con<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct SeqTerm<'a> {
    pub items: Vec<Term<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct ConTerm<'a> {
    pub name: Name<'a>,
    pub args: Vec<Term<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Opr<'a> {
    pub left: Term<'a>,
    pub name: Name<'a>,
    pub right: Term<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct App<'a> {
    pub head: Term<'a>,
    pub children: Vec<Term<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Cond<'a> {
    pub cond: Term<'a>,
    pub left: Term<'a>,
    pub right: Term<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Abs<'a> {
    pub args: Vec<Name<'a>>,
    pub body: Term<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Branch<'a> {
    pub con: Name<'a>,
    pub tpl: Tpl<'a>,
    pub body: Term<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Case<'a> {
    pub cond: Term<'a>,
    pub branches: Vec<Branch<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Let<'a> {
    pub tpls: Vec<Tpl<'a>>,
    pub terms: Vec<Term<'a>>,
    pub body: Term<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Letrec<'a> {
    pub tpls: Vec<Tpl<'a>>,
    pub terms: Vec<Term<'a>>,
    pub body: Term<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum TermNode<'a> {
    Var(Name<'a>),
    Lit(Lit<'a>),
    Seq(SeqTerm<'a>),
    Con(ConTerm<'a>),
    Opr(Opr<'a>),
    App(App<'a>),
    Cond(Cond<'a>),
    Abs(Abs<'a>),
    Case(Case<'a>),
    Let(Let<'a>),
    Letrec(Letrec<'a>),
}

impl<'a> TermNode<'a> {
    pub fn ptr(self) -> Term<'a> {
        Box::new(self)
    }
}

#[derive(Debug, new)]
pub struct SeqTpl<'a> {
    pub items: Vec<Tpl<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct AsTpl<'a> {
    pub name: Name<'a>,
    pub tpl: Tpl<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum TplNode<'a> {
    Var(Name<'a>),
    Seq(SeqTpl<'a>),
    As(AsTpl<'a>),
}

impl<'a> TplNode<'a> {
    pub fn ptr(self) -> Tpl<'a> {
        Box::new(self)
    }
}

pub type Term<'a> = Box<TermNode<'a>>;
pub type Tpl<'a> = Box<TplNode<'a>>;
