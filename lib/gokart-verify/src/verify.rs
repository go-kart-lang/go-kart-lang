use std::collections::HashSet;

use crate::{
    ctx::{Ctx, TypeExt},
    err::{VerifyErr, VerifyRes},
};
use gokart_core::{
    Abs, App, Ast, Branch, Case, ConTerm, Cond, Def, EmptyTerm, Let, Letrec, Lit, LocExt, Name,
    Opr, PairTerm, Tag, Term, Type, TypeDef, TypeIdx,
};

trait Verify<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type>;
}

impl<'a> Verify<'a> for EmptyTerm<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        Ok(ctx.unit_ty())
    }
}

impl<'a> Verify<'a> for PairTerm<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        Ok(Type::pair(self.left.verify(ctx)?, self.right.verify(ctx)?))
    }
}

impl<'a> Verify<'a> for Name<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        Ok(ctx.var(self)?.clone())
    }
}

impl<'a> Verify<'a> for Lit<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        match self {
            Lit::Int(_) => Ok(ctx.int_ty()),
            Lit::Double(_) => Ok(ctx.double_ty()),
            Lit::Str(_) => Ok(ctx.str_ty()),
        }
    }
}

impl<'a> Verify<'a> for ConTerm<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let ty = self.body.verify(ctx)?;
        let (expected_ty, new_ty) = ctx.ctor(&self.name)?;

        ctx.resolve_apply(&expected_ty, &ty, &self.body.loc())?;
        Ok(Type::Prim(new_ty))
    }
}

impl<'a> Verify<'a> for Opr<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let left_ty = self.left.verify(ctx)?;
        let right_ty = self.right.verify(ctx)?;
        let (expected_left_ty, expected_right_ty, res_ty) = ctx.opr(&self.name)?;

        ctx.resolve_apply(&expected_left_ty, &left_ty, &self.left.loc())?;
        ctx.resolve_apply(&expected_right_ty, &right_ty, &self.right.loc())?;
        Ok(res_ty)
    }
}

impl<'a> Verify<'a> for App<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let head_ty = self.head.verify(ctx)?;
        let body_ty = self.body.verify(ctx)?;

        match head_ty {
            Type::Func(from_ty, into_ty) => {
                let subst = ctx.resolve_apply(&from_ty, &body_ty, &self.body.loc())?;
                Ok(into_ty.apply(&subst))
            }
            Type::Free(idx) => match body_ty.contains_ft(idx) {
                false => {
                    let ft = ctx.next_ft();
                    ctx.resolve_apply(
                        &Type::Free(idx),
                        &Type::Func(body_ty.clone().ptr(), ft.clone().ptr()),
                        &self.head.loc(),
                    )?;
                    Ok(ft)
                }
                true => Err(VerifyErr::InfiniteType(self.head.loc().into_span())),
            },
            ty => {
                let ft = ctx.next_ft().ptr();
                Err(VerifyErr::TypeMismatch(
                    self.loc.into_span(),
                    Type::Func(body_ty.ptr(), ft).show(ctx),
                    ty.show(ctx),
                ))
            }
        }
    }
}

impl<'a> Verify<'a> for Cond<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let cond_ty = self.cond.verify(ctx)?;
        let left_ty = self.left.verify(ctx)?;
        let right_ty = self.right.verify(ctx)?;

        ctx.resolve_apply(&ctx.int_ty(), &cond_ty, &self.loc)?;
        ctx.resolve_apply(&left_ty, &right_ty, &self.loc)?;

        Ok(left_ty)
    }
}

impl<'a> Verify<'a> for Abs<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let ft = ctx.next_ft();
        let prev_ty = ctx.push_var(&self.arg, ft.clone());
        let body_ty = self.body.verify(ctx)?;
        let arg_ty = ctx.var(&self.arg)?.clone().ptr();
        ctx.pop_var(&self.arg, prev_ty);

        Ok(Type::Func(arg_ty, body_ty.ptr()))
    }
}

fn verify_branch<'a>(branch: &Branch<'a>, ctx: &mut Ctx<'a>) -> VerifyRes<(Tag, TypeIdx, Type)> {
    let (from_ty, into_ty) = ctx.ctor(&branch.con)?;
    let tag = ctx.tag(&branch.con)?;

    let (vars, tys) = from_ty.split(&branch.tpl, ctx)?;
    let prev_vars = ctx.push_vars(&vars, tys);

    let res = branch.body.verify(ctx)?;

    ctx.pop_vars(&vars, prev_vars);
    Ok((tag, into_ty, res))
}

impl<'a> Verify<'a> for Case<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let mut it = self.branches.iter();
        let mut tags = HashSet::new();

        let (into_ty, res_ty) = match it.next() {
            Some(first) => {
                let (tag, into_ty, res_ty) = verify_branch(first, ctx)?;
                tags.insert(tag);

                it.try_fold((into_ty, res_ty), |(into_ty, res_ty), branch| {
                    let (tag, branch_into_ty, branch_ty) = verify_branch(branch, ctx)?;

                    let span = branch.loc.into_span();
                    if !tags.insert(tag) {
                        return Err(VerifyErr::BranchRedefinition(span));
                    }
                    if into_ty != branch_into_ty {
                        return Err(VerifyErr::InvalidBranchesType(span));
                    }

                    ctx.resolve_apply(&res_ty, &branch_ty, &branch.loc)?;
                    Ok((into_ty, res_ty))
                })?
            }
            None => unreachable!(
                "We always have at least one branch. This is checked at the parsing stage"
            ),
        };

        let cons = ctx.ty_cons(into_ty);
        if &tags != cons {
            return Err(VerifyErr::BranchNotCovered(self.loc.into_span()));
        }

        let cond_ty = self.cond.verify(ctx)?;
        ctx.resolve_apply(&Type::Prim(into_ty), &cond_ty, &self.cond.loc())?;

        Ok(res_ty)
    }
}

impl<'a> Verify<'a> for Let<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let ty = self.term.verify(ctx)?;

        let (vars, tys) = ty.split(&self.tpl, ctx)?;
        let prev_vars = ctx.push_vars(&vars, tys);

        let res = self.body.verify(ctx);

        ctx.pop_vars(&vars, prev_vars);
        res
    }
}

impl<'a> Verify<'a> for Letrec<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        let ft = ctx.next_ft();

        let (vars, fts) = ft.clone().split(&self.tpl, ctx)?;
        let prev_vars = ctx.push_vars(&vars, fts.clone());

        let ty = self.term.verify(ctx)?;
        let (_, tys) = ty.split(&self.tpl, ctx)?;

        fts.iter()
            .zip(tys.iter())
            .map(|(ft, ty)| ctx.resolve_apply(ft, ty, &self.loc))
            .collect::<VerifyRes<Vec<_>>>()?;

        let res = self.body.verify(ctx);

        ctx.pop_vars(&vars, prev_vars);
        res
    }
}

impl<'a> Verify<'a> for Term<'a> {
    fn verify(&self, ctx: &mut Ctx<'a>) -> VerifyRes<Type> {
        match self {
            Term::Empty(term) => term.verify(ctx),
            Term::Pair(term) => term.verify(ctx),
            Term::Var(term) => term.verify(ctx),
            Term::Lit(term) => term.verify(ctx),
            Term::Con(term) => term.verify(ctx),
            Term::Opr(term) => term.verify(ctx),
            Term::App(term) => term.verify(ctx),
            Term::Cond(term) => term.verify(ctx),
            Term::Abs(term) => term.verify(ctx),
            Term::Case(term) => term.verify(ctx),
            Term::Let(term) => term.verify(ctx),
            Term::Letrec(term) => term.verify(ctx),
        }
    }
}

pub trait Apply<'a> {
    fn apply(&self, ctx: &mut Ctx<'a>) -> VerifyRes<()>;
}

impl<'a> Apply<'a> for TypeDef<'a> {
    fn apply(&self, ctx: &mut Ctx<'a>) -> VerifyRes<()> {
        let uty = ctx.add_ty(&self.name)?;

        for con in self.cons.iter() {
            let mut it = con.args.iter();
            let from = match it.next() {
                Some(init) => it.fold(ctx.prim_ty(init), |acc, p| {
                    Ok(Type::Pair(acc?.ptr(), ctx.prim_ty(p)?.ptr()))
                })?,
                None => ctx.unit_ty(),
            };
            ctx.add_ctor(&con.name, from, uty)?;
        }
        Ok(())
    }
}

impl<'a> Apply<'a> for Def<'a> {
    fn apply(&self, ctx: &mut Ctx<'a>) -> VerifyRes<()> {
        match self {
            Def::TypeDef(type_def) => type_def.apply(ctx),
        }
    }
}

pub fn verify(ast: &Ast) -> VerifyRes<()> {
    let mut ctx = Ctx::with_predef();

    for def in ast.defs.iter() {
        def.apply(&mut ctx)?;
    }
    ast.body.verify(&mut ctx)?;

    Ok(())
}
