use crate::{ctx::Ctx, err::VerifyRes};
use gokart_core::{Def, TypeDef};

pub trait Apply<'a> {
    fn apply(&self, ctx: Ctx<'a>) -> VerifyRes<'a, Ctx<'a>>;
}

impl<'a> Apply<'a> for TypeDef<'a> {
    fn apply(&self, ctx: Ctx<'a>) -> VerifyRes<'a, Ctx<'a>> {
        Ok(ctx) // todo
    }
}

impl<'a> Apply<'a> for Def<'a> {
    fn apply(&self, ctx: Ctx<'a>) -> VerifyRes<'a, Ctx<'a>> {
        match self {
            Def::TypeDef(type_def) => type_def.apply(ctx),
        }
    }
}
