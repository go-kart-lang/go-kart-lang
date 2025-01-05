use crate::{
    err::{LogicErr, LogicRes},
    scope::{Names, Scope},
};
use gokart_core::{
    Ast, Def, Exp, ExpNode, InfixDef, LetKind, Lit, Pat, PatNode, PrimOp, Sys, Term, TermNode, Tpl,
    TplNode, TypeDef,
};
use std::ops::Deref;

trait AsExp<'a> {
    fn as_exp(self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp>;
}

impl<'a, 'b, T> AsExp<'a> for &'b Vec<T>
where
    &'b T: AsExp<'a>,
{
    fn as_exp(self, sc: &mut Scope<'a>) -> LogicRes<'a, Exp> {
        let mut it = self.iter();
        match it.next() {
            Some(init) => it.fold(init.as_exp(sc), |acc, term| {
                Ok(ExpNode::Pair(acc?, term.as_exp(sc)?).ptr())
            }),
            None => Ok(ExpNode::Empty.ptr()),
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
            TermNode::Lit(lit) => {
                let sys = match *lit {
                    Lit::Int(val) => Sys::IntLit(val),
                    Lit::Double(val) => Sys::DoubleLit(val),
                    Lit::Str(val) => Sys::StrLit(String::from(val)),
                };
                Ok(ExpNode::Sys(sys).ptr())
            }
            TermNode::Seq(terms) => terms.as_exp(sc),
            TermNode::Con(name, terms) => {
                let tag = sc.tag(&name.span)?;
                Ok(ExpNode::Con(tag, terms.as_exp(sc)?).ptr())
            }
            TermNode::Opr(left, opr, right) => {
                // todo
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
                let names = Names::new().make(&params)?;
                names.with_scope(sc, |s| {
                    Ok(ExpNode::Abs(params.as_pat(s)?, body.as_exp(s)?).ptr())
                })
            }
            TermNode::Case(body, branches) => {
                let body = body.as_exp(sc)?;
                let branches = branches
                    .into_iter()
                    .map(|(con, tpl, term)| {
                        let ctor = sc.tag(&con.span)?;
                        let names = Names::new().make(&tpl)?;
                        names.with_scope(sc, |s| {
                            let pat = tpl.as_pat(s)?;
                            let exp = term.as_exp(s)?;
                            Ok((ctor, pat, exp))
                        })
                    })
                    .collect::<LogicRes<Vec<_>>>()?;
                Ok(ExpNode::Case(body, branches).ptr())
            }
            TermNode::Let(kind, tpl, term, body) => {
                let names = Names::new().make(&tpl)?;
                match kind {
                    LetKind::NonRec => {
                        let body = body.as_exp(sc)?;
                        let (pat, exp) = names.with_scope(sc, |s| {
                            let pat = tpl.as_pat(s)?;
                            let exp = term.as_exp(s)?;
                            Ok((pat, exp))
                        })?;
                        Ok(ExpNode::Let(pat, exp, body).ptr())
                    }
                    LetKind::Rec => names.with_scope(sc, |s| {
                        let pat = tpl.as_pat(s)?;
                        let exp = term.as_exp(s)?;
                        let body = body.as_exp(s)?;
                        Ok(ExpNode::Letrec(pat, exp, body).ptr())
                    }),
                }
            }
        }
    }
}

trait AsPat<'a> {
    fn as_pat(self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat>;
}

impl<'a, 'b, T> AsPat<'a> for &'b Vec<T>
where
    &'b T: AsPat<'a>,
{
    fn as_pat(self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat> {
        let mut it = self.iter();
        match it.next() {
            Some(init) => it.fold(init.as_pat(sc), |acc, tpl| {
                Ok(PatNode::Pair(acc?, tpl.as_pat(sc)?).ptr())
            }),
            None => Ok(PatNode::Empty.ptr()),
        }
    }
}

impl<'a> AsPat<'a> for &Tpl<'a> {
    fn as_pat(self, sc: &mut Scope<'a>) -> LogicRes<'a, Pat> {
        match self.deref() {
            TplNode::Var(var) => {
                let idx = sc.var(&var.span)?;
                Ok(PatNode::Var(idx).ptr())
            }
            TplNode::Empty => Ok(PatNode::Empty.ptr()),
            TplNode::Seq(tpls) => tpls.as_pat(sc),
            TplNode::As(var, tpl) => {
                let idx = sc.var(&var.span)?;
                Ok(PatNode::Layer(idx, tpl.as_pat(sc)?).ptr())
            }
        }
    }
}

trait Introduce<'a> {
    fn introduce(self, sc: &mut Scope<'a>) -> LogicRes<'a, ()>;
}

impl<'a> Introduce<'a> for &TypeDef<'a> {
    fn introduce(self, sc: &mut Scope<'a>) -> LogicRes<'a, ()> {
        self.cons.iter().fold(Ok(()), |acc, con| {
            acc?;
            sc.add_tag(&con.name.span)
        })
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

pub fn decay<'a>(ast: Ast<'a>) -> LogicRes<'a, Exp> {
    let mut sc = Scope::new();

    for def in ast.defs.iter() {
        def.introduce(&mut sc)?;
    }
    ast.body.as_exp(&mut sc)
}
