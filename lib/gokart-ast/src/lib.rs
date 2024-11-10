use ordered_float::NotNan;
use std::{cell::RefCell, rc::Rc};

pub type Var = String;
pub type Con = u32;

#[derive(Debug)]
pub enum Exp {
    Variable(Var),
    ConstInt(i32),
    Sys(String, Box<Exp>, Box<Exp>),
    EmptyTuple,
    Pair(Box<Exp>, Box<Exp>),
    Constructor(Con, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
    Abstraction(Pat, Box<Exp>),
    Conditional(Box<Exp>, Box<Exp>, Box<Exp>),
    Case(Box<Exp>, Vec<(Con, Pat, Box<Exp>)>),
    Local(Pat, Box<Exp>, Box<Exp>),
    LocalRec(Pat, Box<Exp>, Box<Exp>),
}

#[derive(Debug)]
pub enum Pat {
    Variable(Var),
    EmptyPattern,
    Pair(Box<Pat>, Box<Pat>),
    Layer(Var, Box<Pat>),
}

type Ptr<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub struct Definitions<'a>(pub Vec<Definition<'a>>);

#[derive(Debug)]
pub enum InfixType {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Definition<'a> {
    Type(TypeDef<'a>),
    Func(FuncDef<'a>),
    Infix(InfixType, &'a str, u64),
}

#[derive(Debug)]
pub struct TypeDef<'a> {
    pub type_name: &'a str,
    pub constructors: Vec<Constructor<'a>>,
}

#[derive(Debug)]
pub struct Constructor<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
}

#[derive(Debug)]
pub struct FuncDef<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Ptr<Ast<'a>>,
}

#[derive(Debug)]
pub enum Ast<'a> {
    Var(&'a str),
    Literal(Literal<'a>),
    Operator(Ptr<Ast<'a>>, &'a str, Ptr<Ast<'a>>),
    App {
        head: Ptr<Ast<'a>>,
        children: Vec<Ptr<Ast<'a>>>,
    },
    IfThenElse(Ptr<Ast<'a>>, Ptr<Ast<'a>>, Ptr<Ast<'a>>),
    Lambda {
        args: Vec<&'a str>,
        body: Ptr<Ast<'a>>,
    },
    Case(Ptr<Ast<'a>>, Vec<(Ptr<PatAst<'a>>, Ptr<Ast<'a>>)>),
    Let(LetType, Vec<Ptr<FuncDef<'a>>>, Ptr<Ast<'a>>),
}

#[derive(Debug)]
pub enum LetType {
    NonRec,
    Rec,
}

#[derive(Debug)]
pub enum PatAst<'a> {
    Var(&'a str),
    Literal(Literal<'a>),
    As(&'a str, Ptr<PatAst<'a>>),
    Constructor(&'a str, Vec<Ptr<PatAst<'a>>>),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Literal<'a> {
    Int(i64),
    Double(NotNan<f64>),
    String(&'a str),
}
