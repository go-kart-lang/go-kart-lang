use crate::{ctx::Ctx, err::DecayRes, state::State};
use gokart_core::{AsTpl, EmptyTpl, Name, PairTpl, Pat, Tpl};

pub trait AsPat<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Pat>;
}

impl<'a> AsPat<'a> for EmptyTpl<'a> {
    fn as_pat(&self, _ctx: &Ctx<'a>, _st: &mut State) -> DecayRes<'a, Pat> {
        Ok(Pat::Empty)
    }
}

impl<'a> AsPat<'a> for Name<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, _st: &mut State) -> DecayRes<'a, Pat> {
        let idx = ctx.var(self)?;
        Ok(Pat::Var(idx))
    }
}

impl<'a> AsPat<'a> for PairTpl<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Pat> {
        Ok(Pat::Pair(
            self.left.as_pat(ctx, st)?.ptr(),
            self.right.as_pat(ctx, st)?.ptr(),
        ))
    }
}

impl<'a> AsPat<'a> for AsTpl<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Pat> {
        let idx = ctx.var(&self.name)?;
        Ok(Pat::Layer(idx, self.tpl.as_pat(ctx, st)?.ptr()))
    }
}

impl<'a> AsPat<'a> for Tpl<'a> {
    fn as_pat(&self, ctx: &Ctx<'a>, st: &mut State) -> DecayRes<'a, Pat> {
        match self {
            Tpl::Empty(tpl) => tpl.as_pat(ctx, st),
            Tpl::Var(tpl) => tpl.as_pat(ctx, st),
            Tpl::Pair(tpl) => tpl.as_pat(ctx, st),
            Tpl::As(tpl) => tpl.as_pat(ctx, st),
        }
    }
}
