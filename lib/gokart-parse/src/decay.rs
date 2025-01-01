use crate::{ast::*, err::LogicRes, scope::Scope};
use gokart_core::{Exp, ExpNode, Pat, PatNode};
use std::ops::Deref;

trait AsExp<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp>;
}

trait AsPat<'a> {
    fn as_pat(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat>;
}

impl<'a> AsPat<'a> for Name<'a> {
    fn as_pat(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat> {
        Ok(PatNode::Var(sc.var(&self.span)?).ptr())
    }
}

impl<'a> AsExp<'a> for Term<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        match self.deref() {
            TermNode::Var(var) => {
                let idx = sc.var(&var.span)?;
                Ok(ExpNode::Var(idx).ptr())
            }
            TermNode::Lit(lit) => todo!(),
            TermNode::Opr(left, opr, right) => todo!(),
            TermNode::App(head, children) => {
                // todo: right?
                let init = head.as_exp(sc);
                children.iter().fold(init, |acc, child| {
                    Ok(ExpNode::App(acc?, child.as_exp(sc)?).ptr())
                })
            }
            TermNode::Cond(cond, left, right) => {
                Ok(ExpNode::Cond(cond.as_exp(sc)?, left.as_exp(sc)?, right.as_exp(sc)?).ptr())
            }
            TermNode::Abs(params, body) => {
                let init = body.as_exp(sc);
                params.iter().fold(init, |acc, param| {
                    let pat = param.as_pat(sc)?;
                    Ok(ExpNode::Abs(pat, acc?).ptr())
                })
            }
            TermNode::Case(body, branches) => todo!(),
            TermNode::Let(kind, funcs, body) => todo!(),
        }
    }
}

impl<'a> AsExp<'a> for TypeDef<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        todo!()
    }
}

impl<'a> AsExp<'a> for FuncDef<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        let init = self.body.as_exp(sc);
        let func = self.params.iter().fold(init, |acc, param| {
            let pat = param.as_pat(sc)?;
            Ok(ExpNode::Abs(pat, acc?).ptr())
        })?;
        sc.funcs.add(&self.name.span, func)?;
        todo!()
    }
}

impl<'a> AsExp<'a> for InfixDef<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        todo!()
    }
}

impl<'a> AsExp<'a> for Def<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        match self {
            Def::TypeDef(type_def) => type_def.as_exp(sc),
            Def::FuncDef(func_def) => func_def.as_exp(sc),
            Def::InfixDef(infix_def) => infix_def.as_exp(sc),
        }
    }
}

impl<'a> AsExp<'a> for Ast<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        for def in self.defs.iter() {
            def.as_exp(sc)?;
        }
        todo!() // get main
    }
}

pub fn decay<'a>(ast: Ast<'a>) -> LogicRes<'a, Exp> {
    let mut sc = Scope::new();
    ast.as_exp(&mut sc)
}