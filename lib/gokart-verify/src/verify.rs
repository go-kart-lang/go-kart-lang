use crate::{
    ctx::{resolve, resolve_apply, Ctx},
    err::{VerifyErr, VerifyRes},
    state::State,
};
use gokart_core::{
    Abs, App, Ast, Case, ConTerm, Cond, Def, EmptyTerm, Let, Letrec, Lit, LocExt, Name, Opr,
    PairTerm, PredefTerm, Subst, Term, Type, TypeDef,
};

trait Verify<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type>;
}

impl<'a> Verify<'a> for EmptyTerm<'a> {
    fn verify(&mut self, _ctx: &mut Ctx<'a>, _st: &mut State) -> VerifyRes<Type> {
        Ok(Type::unit())
    }
}

impl<'a> Verify<'a> for PairTerm<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        Ok(Type::Pair(
            self.left.verify(ctx, st)?.ptr(),
            self.right.verify(ctx, st)?.ptr(),
        ))
    }
}

impl<'a> Verify<'a> for Name<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, _st: &mut State) -> VerifyRes<Type> {
        Ok(ctx.var(self)?.clone())
    }
}

impl<'a> Verify<'a> for Lit<'a> {
    fn verify(&mut self, _ctx: &mut Ctx<'a>, _st: &mut State) -> VerifyRes<Type> {
        match self {
            Lit::Int(_) => Ok(Type::int()),
            Lit::Double(_) => Ok(Type::double()),
            Lit::Str(_) => Ok(Type::str()),
        }
    }
}

impl<'a> Verify<'a> for ConTerm<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        let ty = self.body.verify(ctx, st)?;
        let (expected_ty, new_ty) = st.ctor(&self.name)?;

        match resolve_apply(ctx, &ty, expected_ty) {
            Ok(_) => Ok(new_ty.clone()),
            Err(msg) => Err(VerifyErr::TypeMismatch(self.loc.into_span(), msg)),
        }
    }
}

impl<'a> Verify<'a> for Opr<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        // let left_ty = self.left.verify(ctx, st)?;
        // let right_ty = self.right.verify(ctx, st)?;

        todo!()
    }
}

impl<'a> Verify<'a> for App<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        let head_ty = self.head.verify(ctx, st)?;
        let body_ty = self.body.verify(ctx, st)?;

        match head_ty {
            Type::Func(from_ty, into_ty) => match resolve(&from_ty, &body_ty) {
                Ok(subst) => Ok(into_ty.apply(&subst)),
                Err(msg) => Err(VerifyErr::TypeMismatch(self.loc.into_span(), msg)),
            },
            Type::Free(idx) => {
                let ft = st.next_ft();
                let subst = Subst::from_iter([(idx, Type::Func(body_ty.ptr(), ft.clone().ptr()))]);
                ctx.apply(&subst);
                Ok(ft)
            }
            // todo: message
            _ => Err(VerifyErr::TypeMismatch(
                self.loc.into_span(),
                "Expect function type".to_string(),
            )),
        }
    }
}

impl<'a> Verify<'a> for Cond<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        let cond_ty = self.cond.verify(ctx, st)?;
        let left_ty = self.left.verify(ctx, st)?;
        let right_ty = self.right.verify(ctx, st)?;

        match resolve_apply(ctx, &cond_ty, &Type::int()) {
            Ok(_) => Ok(()),
            Err(msg) => Err(VerifyErr::TypeMismatch(self.loc.into_span(), msg)),
        }?;

        match resolve_apply(ctx, &left_ty, &right_ty) {
            Ok(_) => Ok(()),
            Err(msg) => Err(VerifyErr::TypeMismatch(self.loc.into_span(), msg)),
        }?;

        Ok(left_ty)
    }
}

impl<'a> Verify<'a> for Abs<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        let ft = st.next_ft();
        let prev_ty = ctx.push_var(&self.arg, ft.clone());
        let body_ty = self.body.verify(ctx, st)?;
        let arg_ty = ctx.var(&self.arg)?.clone().ptr();
        ctx.pop_var(&self.arg, prev_ty);

        Ok(Type::Func(arg_ty, body_ty.ptr()))
    }
}

impl<'a> Verify<'a> for Case<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        todo!()
    }
}

impl<'a> Verify<'a> for Let<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        let ty = self.term.verify(ctx, st)?;
        let arg = self.tpl.as_var(); // todo

        let prev_ty = ctx.push_var(&arg, ty);
        let res = self.body.verify(ctx, st);
        ctx.pop_var(&arg, prev_ty);

        res
    }
}

impl<'a> Verify<'a> for Letrec<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        todo!()
    }
}

impl<'a> Verify<'a> for PredefTerm<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        for (name, _, ty) in self.predef.items.iter() {
            ctx.push_var(name, ty.clone());
        }

        self.body.verify(ctx, st)
    }
}

impl<'a> Verify<'a> for Term<'a> {
    fn verify(&mut self, ctx: &mut Ctx<'a>, st: &mut State) -> VerifyRes<Type> {
        match self {
            Term::Empty(term) => term.verify(ctx, st),
            Term::Pair(term) => term.verify(ctx, st),
            Term::Var(term) => term.verify(ctx, st),
            Term::Lit(term) => term.verify(ctx, st),
            Term::Con(term) => term.verify(ctx, st),
            Term::Opr(term) => term.verify(ctx, st),
            Term::App(term) => term.verify(ctx, st),
            Term::Cond(term) => term.verify(ctx, st),
            Term::Abs(term) => term.verify(ctx, st),
            Term::Case(term) => term.verify(ctx, st),
            Term::Let(term) => term.verify(ctx, st),
            Term::Letrec(term) => term.verify(ctx, st),
            Term::Predef(term) => term.verify(ctx, st),
        }
    }
}

pub trait Apply<'a> {
    fn apply(&self, st: &mut State<'a>) -> VerifyRes<()>;
}

impl<'a> Apply<'a> for TypeDef<'a> {
    fn apply(&self, st: &mut State<'a>) -> VerifyRes<()> {
        let uty = st.add_ty(&self.name)?;

        for con in self.cons.iter() {
            let mut it = con.args.iter();
            let from = match it.next() {
                Some(init) => it.fold(st.ty(init), |acc, p| {
                    Ok(Type::Pair(acc?.ptr(), st.ty(p)?.ptr()))
                })?,
                None => Type::unit(),
            };
            st.add_ctor(&con.name, from, uty.clone())?;
        }
        Ok(())
    }
}

impl<'a> Apply<'a> for Def<'a> {
    fn apply(&self, st: &mut State<'a>) -> VerifyRes<()> {
        match self {
            Def::TypeDef(type_def) => type_def.apply(st),
        }
    }
}

pub fn verify(ast: &mut Ast) -> VerifyRes<()> {
    let mut ctx = Ctx::new();
    let mut st = State::new();

    for def in ast.defs.iter() {
        def.apply(&mut st)?;
    }
    ast.body.verify(&mut ctx, &mut st)?;

    Ok(())
}
