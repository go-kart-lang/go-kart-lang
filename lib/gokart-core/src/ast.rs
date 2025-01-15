use crate::Loc;
use derive_new::new;
use std::ops::Deref;

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

#[derive(Debug, new)]
pub struct BoolLit<'a> {
    pub val: bool,
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum Lit<'a> {
    Int(IntLit<'a>),
    Double(DoubleLit<'a>),
    Str(StrLit<'a>),
}

impl<'a> Lit<'a> {
    pub fn loc(&self) -> Loc<'a> {
        match self {
            Lit::Int(lit) => lit.loc,
            Lit::Double(lit) => lit.loc,
            Lit::Str(lit) => lit.loc,
        }
    }
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
pub struct EmptyTerm<'a> {
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct SysTerm<'a> {
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct PairTerm<'a> {
    pub left: TermPtr<'a>,
    pub right: TermPtr<'a>,
    pub loc: Loc<'a>,
}

pub type VarName<'a> = &'a str;

#[derive(Debug, new, Clone)]
pub struct Name<'a> {
    pub val: VarName<'a>,
    pub loc: Loc<'a>,
}

impl<'a> Deref for Name<'a> {
    type Target = VarName<'a>;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Debug, new)]
pub struct Con<'a> {
    pub name: Name<'a>,
    pub args: Vec<Name<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct TypeDef<'a> {
    pub name: Name<'a>,
    pub cons: Vec<Con<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct ConTerm<'a> {
    pub name: Name<'a>,
    pub body: TermPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Opr<'a> {
    pub left: TermPtr<'a>,
    pub name: Name<'a>,
    pub right: TermPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct App<'a> {
    pub head: TermPtr<'a>,
    pub body: TermPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Cond<'a> {
    pub cond: TermPtr<'a>,
    pub left: TermPtr<'a>,
    pub right: TermPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Abs<'a> {
    pub arg: Name<'a>,
    pub body: TermPtr<'a>,
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
    pub cond: TermPtr<'a>,
    pub branches: Vec<Branch<'a>>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Let<'a> {
    pub tpl: Tpl<'a>,
    pub term: TermPtr<'a>,
    pub body: TermPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct Letrec<'a> {
    pub tpl: Tpl<'a>,
    pub term: TermPtr<'a>,
    pub body: TermPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum Term<'a> {
    Empty(EmptyTerm<'a>),
    Pair(PairTerm<'a>),
    Var(Name<'a>),
    Lit(Lit<'a>),
    Con(ConTerm<'a>),
    Opr(Opr<'a>),
    App(App<'a>),
    Cond(Cond<'a>),
    Abs(Abs<'a>),
    Case(Case<'a>),
    Let(Let<'a>),
    Letrec(Letrec<'a>),
}

impl<'a> Term<'a> {
    #[inline]
    pub fn ptr(self) -> TermPtr<'a> {
        Box::new(self)
    }

    pub fn loc(&self) -> Loc<'a> {
        match self {
            Term::Empty(term) => term.loc,
            Term::Pair(term) => term.loc,
            Term::Var(term) => term.loc,
            Term::Lit(term) => term.loc(),
            Term::Con(term) => term.loc,
            Term::Opr(term) => term.loc,
            Term::App(term) => term.loc,
            Term::Cond(term) => term.loc,
            Term::Abs(term) => term.loc,
            Term::Case(term) => term.loc,
            Term::Let(term) => term.loc,
            Term::Letrec(term) => term.loc,
        }
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
    pub tpl: TplPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct PairTpl<'a> {
    pub left: TplPtr<'a>,
    pub right: TplPtr<'a>,
    pub loc: Loc<'a>,
}

#[derive(Debug, new)]
pub struct EmptyTpl<'a> {
    pub loc: Loc<'a>,
}

#[derive(Debug)]
pub enum Tpl<'a> {
    Empty(EmptyTpl<'a>),
    Var(Name<'a>),
    Pair(PairTpl<'a>),
    As(AsTpl<'a>),
}

impl<'a> Tpl<'a> {
    #[inline]
    pub fn ptr(self) -> TplPtr<'a> {
        Box::new(self)
    }

    pub fn loc(&self) -> Loc<'a> {
        match self {
            Tpl::Empty(tpl) => tpl.loc,
            Tpl::Var(tpl) => tpl.loc,
            Tpl::Pair(tpl) => tpl.loc,
            Tpl::As(tpl) => tpl.loc,
        }
    }
}

pub type TermPtr<'a> = Box<Term<'a>>;
pub type TplPtr<'a> = Box<Tpl<'a>>;
