use crate::{
    ast::*,
    err::{LogicErr, LogicRes},
    scope::Scope,
};
use gokart_core::{Exp, ExpNode, Pat, PatNode, PrimOp, Sys};
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

impl<'a> AsPat<'a> for Tpl<'a> {
    fn as_pat(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat> {
        match self.deref() {
            TplNode::As(var, tpl) => {
                let idx = sc.var(&var.span)?;
                Ok(PatNode::Layer(idx, tpl.as_pat(sc)?).ptr())
            }
            TplNode::Var(var) => var.as_pat(sc),
            TplNode::Lit(lit) => todo!(),
            TplNode::Con(name, vec) => todo!(),
        }
    }
}

impl<'a> AsExp<'a> for Term<'a> {
    fn as_exp(&self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        match self.deref() {
            TermNode::Var(var) => {
                let idx = sc.var(&var.span)?;
                Ok(ExpNode::Var(idx).ptr())
            }
            TermNode::Lit(lit) => Ok(match *lit {
                Lit::Int(val) => ExpNode::Sys(Sys::IntLit(val)).ptr(),
                Lit::Double(val) => todo!(),
                Lit::Str(val) => todo!(),
            }),
            TermNode::Opr(left, opr, right) => {
                let op_kind = match PrimOp::try_from(*opr.fragment()) {
                    Ok(kind) => Ok(kind),
                    Err(m) => Err(LogicErr::new(*opr, m)),
                };
                let prim_op = Sys::PrimOp(left.as_exp(sc)?, op_kind?, right.as_exp(sc)?);
                Ok(ExpNode::Sys(prim_op).ptr())
            }
            TermNode::App(head, children) => {
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
                params.iter().rfold(init, |acc, param| {
                    let pat = param.as_pat(sc)?;
                    Ok(ExpNode::Abs(pat, acc?).ptr())
                })
            }
            TermNode::Case(body, branches) => todo!(),
            TermNode::Let(kind, funcs, body) => {
                todo!()
                // let body = body.as_exp(sc)?;
                // Ok(match kind {
                //     LetKind::NonRec => ExpNode::Let(_, _, body).ptr(),
                //     LetKind::Rec => ExpNode::Letrec(_, _, body).ptr(),
                // })
            }
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
