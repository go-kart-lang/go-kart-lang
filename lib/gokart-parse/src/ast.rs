use derive_new::new;

use crate::token::Span;

#[derive(Debug)]
pub enum InfixKind {
    Left,
    Right,
}

#[derive(Debug)]
pub enum LetKind {
    NonRec,
    Rec,
}

#[derive(Debug)]
pub enum Lit<'a> {
    Int(i64),
    Double(f64),
    Str(&'a str),
}

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
pub struct Name<'a> {
    pub span: Span<'a>,
}

impl<'a> Name<'a> {
    #[inline]
    pub fn val(&self) -> &'a str {
        self.span.fragment()
    }
}

#[derive(Debug, new)]
pub struct Con<'a> {
    pub name: Name<'a>,
    pub params: Vec<Name<'a>>,
}

#[derive(Debug, new)]
pub struct TypeDef<'a> {
    pub name: Name<'a>,
    pub cons: Vec<Con<'a>>,
}

#[derive(Debug, new)]
pub struct FuncDef<'a> {
    pub name: Name<'a>,
    pub params: Vec<Name<'a>>,
    pub body: Term<'a>,
}

#[derive(Debug, new)]
pub struct InfixDef<'a> {
    pub kind: InfixKind,
    pub opr: Span<'a>,
    pub priority: u64, // todo
}

#[derive(Debug)]
pub enum TermNode<'a> {
    Var(Name<'a>),
    Lit(Lit<'a>),
    Opr(Term<'a>, Span<'a>, Term<'a>),
    App(Term<'a>, Vec<Term<'a>>),
    Cond(Term<'a>, Term<'a>, Term<'a>),
    Abs(Vec<Name<'a>>, Term<'a>),
    Case(Term<'a>, Vec<(Tpl<'a>, Term<'a>)>),
    Let(LetKind, Vec<FuncDef<'a>>, Term<'a>),
}

impl<'a> TermNode<'a> {
    pub fn ptr(self) -> Term<'a> {
        Box::new(self)
    }
}

#[derive(Debug)]
pub enum TplNode<'a> {
    As(Name<'a>, Tpl<'a>),
    Var(Name<'a>),
    Lit(Lit<'a>),
    Con(Name<'a>, Vec<Tpl<'a>>),
}

impl<'a> TplNode<'a> {
    pub fn ptr(self) -> Tpl<'a> {
        Box::new(self)
    }
}

pub type Term<'a> = Box<TermNode<'a>>;
pub type Tpl<'a> = Box<TplNode<'a>>;
