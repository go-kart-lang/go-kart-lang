use crate::{
    ast::*,
    err::{LogicErr, LogicRes},
    scope::{Names, Scope},
};
use gokart_core::{Exp, ExpNode, Pat, PatNode, PrimOp, Sys};
use std::ops::Deref;

trait AsExp<'a> {
    fn as_exp(self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp>;
}

trait AsPat<'a> {
    fn as_pat(self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat>;
}

trait Introduce<'a> {
    fn introduce(self, sc: &mut Scope<'a>) -> LogicRes<'a, ()>;
}

impl<'a> AsPat<'a> for &Tpl<'a> {
    fn as_pat(self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat> {
        match self.deref() {
            TplNode::Var(var) => {
                let idx = sc.var(&var.span)?;
                Ok(PatNode::Var(idx).ptr())
            }
            TplNode::Empty => Ok(PatNode::Empty.ptr()),
            TplNode::Seq(tpls) => tpls.iter().as_pat(sc),
            TplNode::As(var, tpl) => {
                let idx = sc.var(&var.span)?;
                Ok(PatNode::Layer(idx, tpl.as_pat(sc)?).ptr())
            }
        }
    }
}

impl<'a, I> AsPat<'a> for I
where
    I: Iterator,
    <I as Iterator>::Item: AsPat<'a>,
{
    fn as_pat(mut self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat> {
        match self.next() {
            None => Ok(PatNode::Empty.ptr()),
            Some(init) => self.fold(init.as_pat(sc), |acc, item| {
                Ok(PatNode::Pair(acc?, item.as_pat(sc)?).ptr())
            }),
        }
    }
}

impl<'a> AsExp<'a> for &Term<'a> {
    fn as_exp(self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
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
            TermNode::Seq(terms) => terms.iter().as_exp(sc),
            TermNode::Con(name, body) => {
                // todo
                let ctor = sc.ctor(&name.span)?;
                Ok(ExpNode::Con(ctor, body.as_exp(sc)?).ptr())
            }
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
                let names = Names::new().collect(&params)?;
                names.with_scope(sc, |s| {
                    Ok(ExpNode::Abs(params.as_pat(s)?, body.as_exp(s)?).ptr())
                })
            }
            TermNode::Case(cond, branches) => {
                // todo: check pat matches con

                let cond = cond.as_exp(sc)?;
                let branches = branches
                    .into_iter()
                    .map(|(con, tpl, term)| {
                        let ctor = sc.ctor(&con.span)?;
                        let names = Names::new().collect(&tpl)?;
                        names.with_scope(sc, |s| {
                            let pat = tpl.as_pat(s)?;
                            let exp = term.as_exp(s)?;
                            Ok((ctor, pat, exp))
                        })
                    })
                    .collect::<LogicRes<Vec<_>>>();
                Ok(ExpNode::Case(cond, branches?).ptr())
            }
            TermNode::Let(kind, tpl, term, body) => {
                let names = Names::new().collect(&tpl)?;
                names.with_scope(sc, |s| {
                    let pat = tpl.as_pat(s)?;
                    let exp = term.as_exp(s)?;
                    let body = body.as_exp(s)?;

                    Ok(match kind {
                        LetKind::NonRec => ExpNode::Let(pat, exp, body).ptr(),
                        LetKind::Rec => ExpNode::Letrec(pat, exp, body).ptr(),
                    })
                })
            }
        }
    }
}

impl<'a, I> AsExp<'a> for I
where
    I: Iterator,
    <I as Iterator>::Item: AsExp<'a>,
{
    fn as_exp(mut self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        match self.next() {
            None => Ok(ExpNode::Empty.ptr()),
            Some(init) => self.fold(init.as_exp(sc), |acc, item| {
                Ok(ExpNode::Pair(acc?, item.as_exp(sc)?).ptr())
            }),
        }
    }
}

impl<'a> Introduce<'a> for &TypeDef<'a> {
    fn introduce(self, sc: &mut Scope<'a>) -> LogicRes<'a, ()> {
        Ok(()) // todo
    }
}

impl<'a> Introduce<'a> for &InfixDef<'a> {
    fn introduce(self, sc: &mut Scope<'a>) -> LogicRes<'a, ()> {
        Ok(()) // todo
    }
}

impl<'a> Introduce<'a> for &Def<'a> {
    fn introduce(self, sc: &mut Scope<'a>) -> LogicRes<'a, ()> {
        match self {
            Def::TypeDef(type_def) => type_def.introduce(sc),
            Def::InfixDef(infix_def) => infix_def.introduce(sc),
        }
    }
}

impl<'a> AsExp<'a> for &Ast<'a> {
    fn as_exp(self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        for def in self.defs.iter() {
            def.introduce(sc)?;
        }
        self.body.as_exp(sc)
    }
}

pub fn decay<'a>(ast: Ast<'a>) -> LogicRes<'a, Exp> {
    let mut sc = Scope::new();
    ast.as_exp(&mut sc)
}
