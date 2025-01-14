use crate::err::{VerifyErr, VerifyRes};
use gokart_core::{LocExt, Name, Subst, Type, VarName};
use std::collections::HashMap;

pub type ResolveRes<T> = Result<T, String>;

#[inline]
pub fn resolve_apply(ctx: &mut Ctx, left: &Type, right: &Type) -> ResolveRes<()> {
    let subst = resolve(left, right)?;
    ctx.apply(&subst);
    Ok(())
}

pub fn resolve(left: &Type, right: &Type) -> ResolveRes<Subst> {
    fn go(subst: &mut Subst, left: &Type, right: &Type) -> ResolveRes<()> {
        match (left, right) {
            (Type::Prim(x), Type::Prim(y)) => {
                if x == y {
                    Ok(())
                } else {
                    // todo
                    Err(format!("Prim {x} not eq {y}"))
                }
            }
            (Type::Pair(a, b), Type::Pair(c, d)) => {
                go(subst, a, c)?;
                go(subst, b, d)
            }
            (Type::Func(a, b), Type::Func(c, d)) => {
                go(subst, a, c)?;
                go(subst, b, d)
            }
            (Type::Free(idx), ty) => {
                subst.insert(*idx, ty.clone());
                Ok(())
            }
            (ty, Type::Free(idx)) => {
                subst.insert(*idx, ty.clone());
                Ok(())
            }
            // todo
            (left, right) => Err(format!("Type {left:?} not eq {right:?}")),
        }
    }

    let mut subst = Subst::new();
    go(&mut subst, left, right)?;
    Ok(subst)
}

#[derive(Debug)]
pub struct Ctx<'a> {
    vars: HashMap<VarName<'a>, Type>,
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    #[inline]
    pub fn var(&self, name: &Name<'a>) -> VerifyRes<&Type> {
        match self.vars.get(name.val) {
            Some(ty) => Ok(ty),
            None => Err(VerifyErr::UnknownName(
                name.loc.into_span(),
                name.val.to_string(),
            )),
        }
    }

    #[inline]
    pub fn push_var(&mut self, name: VarName<'a>, ty: Type) -> Option<Type> {
        self.vars.insert(name, ty)
    }

    #[inline]
    pub fn pop_var(&mut self, name: &Name<'a>, prev_ty: Option<Type>) {
        match prev_ty {
            Some(prev_ty) => self.push_var(name, prev_ty),
            None => None,
        };
    }

    #[inline]
    pub fn apply(&mut self, subst: &Subst) {
        for ty in self.vars.values_mut() {
            *ty = ty.apply(subst);
        }
    }
}
