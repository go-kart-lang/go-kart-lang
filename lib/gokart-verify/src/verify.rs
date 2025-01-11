use crate::{
    apply::Apply,
    ctx::{Ctx, Subst, TypeInfo, TypeScheme},
    err::VerifyRes,
    state::State,
};
use gokart_core::{
    Abs, App, Ast, Case, ConTerm, Cond, EmptyTerm, Let, Letrec, Lit, Name, Opr, PairTerm, Term,
    Type,
};

trait Verify<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo>;
}

impl<'a> Verify<'a> for EmptyTerm<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for PairTerm<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for Name<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        let TypeScheme { tvs, ty } = ctx.lookup(self)?.clone();
        let new_tvs = st.next_tvs(tvs.len());
        let subst = tvs
            .into_iter()
            .zip(new_tvs.into_iter().map(Type::Var))
            .collect::<Subst>();
        Ok(TypeInfo::new(Subst::identity(), subst.ty(ty)))
    }
}

impl<'a> Verify<'a> for Lit<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for ConTerm<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for Opr<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for App<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for Cond<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for Abs<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        //         let beta = Type::Var(ctx.make_tv());
        //         let scheme = TypeScheme::empty(beta.clone());
        //         self.body
        todo!()
    }
}

impl<'a> Verify<'a> for Case<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for Let<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for Letrec<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
        todo!()
    }
}

impl<'a> Verify<'a> for Term<'a> {
    fn verify(&mut self, ctx: &Ctx<'a>, st: &mut State) -> VerifyRes<'a, TypeInfo> {
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
        }
    }
}

pub fn verify(mut ast: Ast) -> VerifyRes<'_, Ast> {
    let mut st = State::new();
    let mut ctx = Ctx::new(); // todo

    for def in ast.defs.iter() {
        ctx = def.apply(ctx)?;
    }
    ast.body.verify(&ctx, &mut st)?;

    Ok(ast)
}
