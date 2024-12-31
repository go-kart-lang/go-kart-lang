use crate::{ast::*, token::Span};
use gokart_core::{Ctor, Exp, ExpNode, Pat, PatNode, Var};
use std::{collections::HashMap, ops::Deref};

// ast -> exp

struct NameTable<'a> {
    vars: HashMap<&'a str, Var>,
    ctors: HashMap<&'a str, Ctor>,
}

impl<'a> NameTable<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            ctors: HashMap::new(),
        }
    }

    pub fn var(&mut self, s: &Span) -> Var {
        todo!()
    }

    pub fn ctor(&mut self, s: &Span) -> Var {
        todo!()
    }
}

trait AsExp<'a> {
    fn as_exp(&self, nt: &mut NameTable<'a>) -> Exp;
}

trait AsPat<'a> {
    fn as_pat(&self, nt: &mut NameTable<'a>) -> Pat;
}

impl<'a> AsExp<'a> for Term<'a> {
    fn as_exp(&self, nt: &mut NameTable<'a>) -> Exp {
        match self.deref() {
            TermNode::Var(name) => ExpNode::Var(nt.var(name)).ptr(),
            TermNode::Lit(lit) => todo!(),
            TermNode::Opr(left, name, right) => todo!(),
            TermNode::App(head, children) => children.iter().fold(head.as_exp(nt), |acc, child| {
                // todo
                ExpNode::App(acc, child.as_exp(nt)).ptr()
            }),
            TermNode::Cond(cond, left, right) => {
                ExpNode::Cond(cond.as_exp(nt), left.as_exp(nt), right.as_exp(nt)).ptr()
            }
            TermNode::Abs(params, body) => params.iter().fold(body.as_exp(nt), |acc, param| {
                let pat = PatNode::Var(nt.var(param)).ptr();
                ExpNode::Abs(pat, acc).ptr()
            }),
            TermNode::Case(cond, branches) => todo!(),
            TermNode::Let(kind, funcs, body) => match kind {
                LetKind::NonRec => todo!(),
                LetKind::Rec => todo!(),
            },
        }
    }
}

impl<'a> AsExp<'a> for TypeDef<'a> {
    fn as_exp(&self, nt: &mut NameTable<'a>) -> Exp {
        todo!()
    }
}

impl<'a> AsExp<'a> for FuncDef<'a> {
    fn as_exp(&self, nt: &mut NameTable<'a>) -> Exp {
        todo!()
    }
}

impl<'a> AsExp<'a> for InfixDef<'a> {
    fn as_exp(&self, nt: &mut NameTable<'a>) -> Exp {
        todo!()
    }
}

impl<'a> AsExp<'a> for Def<'a> {
    fn as_exp(&self, nt: &mut NameTable<'a>) -> Exp {
        match self {
            Def::TypeDef(type_def) => type_def.as_exp(nt),
            Def::FuncDef(func_def) => func_def.as_exp(nt),
            Def::InfixDef(infix_def) => infix_def.as_exp(nt),
        }
    }
}

impl<'a> AsExp<'a> for Ast<'a> {
    fn as_exp(&self, nt: &mut NameTable<'a>) -> Exp {
        self.defs.iter().fold(ExpNode::Empty.ptr(), |acc, def| {
            ExpNode::Pair(acc, def.as_exp(nt)).ptr()
        })
    }
}

pub fn decay<'a>(ast: Ast<'a>) -> Exp {
    let mut nt = NameTable::new();
    ast.as_exp(&mut nt)
}
