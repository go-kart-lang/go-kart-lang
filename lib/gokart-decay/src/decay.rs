use crate::{apply::Apply, as_pat::AsPat, ctx::Ctx, err::DecayRes, state::State};
use gokart_core::{
    Abs, App, Ast, Case, ConTerm, Cond, EmptyTerm, Exp, Let, Letrec, Lit, Name, Opr, PairTerm, Sys,
    Term,
};

trait Decay<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp>;
}

impl<'a> Decay<'a> for EmptyTerm<'a> {
    fn decay(&self, _ctx: &Ctx<'a>, _st: &mut State) -> DecayRes<'a, Exp> {
        Ok(Exp::Empty)
    }
}

impl<'a> Decay<'a> for PairTerm<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        Ok(Exp::Pair(
            self.left.decay(ctx, st)?.ptr(),
            self.right.decay(ctx, st)?.ptr(),
        ))
    }
}

impl<'a> Decay<'a> for Name<'a> {
    fn decay(&self, ctx: &Ctx<'a>, _st: &mut State) -> DecayRes<'a, Exp> {
        let idx = ctx.var(self)?;
        Ok(Exp::Var(idx))
    }
}

impl<'a> Decay<'a> for Lit<'a> {
    fn decay(&self, _ctx: &Ctx<'a>, _st: &mut State) -> DecayRes<'a, Exp> {
        let sys = match self {
            Lit::Int(lit) => Sys::IntLit(lit.val),
            Lit::Double(lit) => Sys::DoubleLit(lit.val),
            Lit::Str(lit) => Sys::StrLit(String::from(lit.val)),
        };
        Ok(Exp::Sys(sys))
    }
}

impl<'a> Decay<'a> for ConTerm<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        let tag = ctx.tag(&self.name)?;
        Ok(Exp::Con(tag, self.body.decay(ctx, st)?.ptr()))
    }
}

impl<'a> Decay<'a> for Opr<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        //                 let op_kind = match PrimOp::try_from(*opr.fragment()) {
        //                     Ok(kind) => Ok(kind),
        //                     Err(m) => Err(LogicErr::new(*opr, m)),
        //                 };
        //                 let prim_op = Sys::PrimOp(left.as_exp(sc)?, op_kind?, right.as_exp(sc)?);
        //                 Ok(ExpNode::Sys(prim_op).ptr())
        todo!()
    }
}

impl<'a> Decay<'a> for App<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        Ok(Exp::App(
            self.head.decay(ctx, st)?.ptr(),
            self.body.decay(ctx, st)?.ptr(),
        ))
    }
}

impl<'a> Decay<'a> for Cond<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        Ok(Exp::Cond(
            self.cond.decay(ctx, st)?.ptr(),
            self.left.decay(ctx, st)?.ptr(),
            self.right.decay(ctx, st)?.ptr(),
        ))
    }
}

impl<'a> Decay<'a> for Abs<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        let ctx_ = ctx.push_var(&self.arg, st);
        Ok(Exp::Abs(
            self.arg.as_pat(&ctx_, st)?,
            self.body.decay(&ctx_, st)?.ptr(),
        ))
    }
}

impl<'a> Decay<'a> for Case<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        let body = self.cond.decay(ctx, st)?;
        let branches = self
            .branches
            .iter()
            .map(|branch| {
                let ctx_ = ctx.push_tpl(&branch.tpl, st);
                let tag = ctx_.tag(&branch.con)?;
                let pat = branch.tpl.as_pat(&ctx_, st)?;
                let exp = branch.body.decay(&ctx_, st)?;

                Ok((tag, pat, exp))
            })
            .collect::<DecayRes<'a, Vec<_>>>()?;

        Ok(Exp::Case(body.ptr(), branches))
    }
}

impl<'a> Decay<'a> for Let<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        let ctx_ = ctx.push_tpl(&self.tpl, st);
        let pat = self.tpl.as_pat(&ctx_, st)?;
        let exp = self.term.decay(ctx, st)?;
        let body = self.body.decay(&ctx_, st)?;
        Ok(Exp::Let(pat, exp.ptr(), body.ptr()))
    }
}

impl<'a> Decay<'a> for Letrec<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        let ctx_ = ctx.push_tpl(&self.tpl, st);
        let pat = self.tpl.as_pat(&ctx_, st)?;
        let exp = self.term.decay(&ctx_, st)?;
        let body = self.body.decay(&ctx_, st)?;
        Ok(Exp::Letrec(pat, exp.ptr(), body.ptr()))
    }
}

impl<'a> Decay<'a> for Term<'a> {
    fn decay(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Exp> {
        match self {
            Term::Empty(term) => term.decay(ctx, st),
            Term::Pair(term) => term.decay(ctx, st),
            Term::Var(term) => term.decay(ctx, st),
            Term::Lit(term) => term.decay(ctx, st),
            Term::Con(term) => term.decay(ctx, st),
            Term::Opr(term) => term.decay(ctx, st),
            Term::App(term) => term.decay(ctx, st),
            Term::Cond(term) => term.decay(ctx, st),
            Term::Abs(term) => term.decay(ctx, st),
            Term::Case(term) => term.decay(ctx, st),
            Term::Let(term) => term.decay(ctx, st),
            Term::Letrec(term) => term.decay(ctx, st),
        }
    }
}

pub fn decay<'a>(ast: &Ast<'a>) -> DecayRes<'a, Exp> {
    let mut st = State::new();
    let mut ctx = Ctx::init_with(&mut st, ["println", "readInt", "readDouble", "readStr"]);

    for def in ast.defs.iter() {
        ctx = def.apply(ctx)?;
    }
    ast.body.decay(&ctx, &mut st)
}
