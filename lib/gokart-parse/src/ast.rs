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

#[derive(Debug, new)]
pub struct InfixPriority {
    // todo: add span
    pub val: u64,
}

#[derive(Debug)]
pub enum Lit<'a> {
    // todo: add span
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
pub struct TypeDef<'a> {
    pub name: Span<'a>,
    pub cons: Vec<(Span<'a>, Vec<Span<'a>>)>,
}

#[derive(Debug, new)]
pub struct FuncDef<'a> {
    pub name: Span<'a>,
    pub params: Vec<Span<'a>>,
    pub body: Term<'a>,
}

#[derive(Debug, new)]
pub struct InfixDef<'a> {
    pub kind: InfixKind,
    pub name: Span<'a>,
    pub priority: InfixPriority, // todo
}

#[derive(Debug)]
pub enum TermNode<'a> {
    Var(Span<'a>),
    Lit(Lit<'a>),
    Opr(Term<'a>, Span<'a>, Term<'a>),
    App(Term<'a>, Vec<Term<'a>>),
    Cond(Term<'a>, Term<'a>, Term<'a>),
    Abs(Vec<Span<'a>>, Term<'a>),
    Case(Term<'a>, Vec<(Tpl<'a>, Term<'a>)>),
    Let(LetKind, Vec<FuncDef<'a>>, Term<'a>),
}

impl<'a> TermNode<'a> {
    pub fn ptr(self) -> Term<'a> {
        Ptr::new(self)
    }
}

#[derive(Debug)]
pub enum TplNode<'a> {
    As(Span<'a>, Tpl<'a>),
    Var(Span<'a>),
    Lit(Lit<'a>),
    Con(Span<'a>, Vec<Tpl<'a>>),
}

impl<'a> TplNode<'a> {
    pub fn ptr(self) -> Tpl<'a> {
        Ptr::new(self)
    }
}

pub type Ptr<T> = Box<T>;
pub type Term<'a> = Ptr<TermNode<'a>>;
pub type Tpl<'a> = Ptr<TplNode<'a>>;
