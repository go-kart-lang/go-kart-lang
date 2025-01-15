use crate::err::{VerifyErr, VerifyRes};
use gokart_core::{
    Counter, FreeIdx, Loc, LocExt, Name, Predef, Subst, Tag, Tpl, Type, TypeIdx, VarName,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Ctx<'a> {
    ty_cnt: Counter,
    ft_cnt: Counter,
    tag_cnt: Counter,
    vars: HashMap<VarName<'a>, Type>,
    ctors: HashMap<VarName<'a>, (Type, TypeIdx)>,
    tags: HashMap<VarName<'a>, Tag>,
    cons: HashMap<TypeIdx, HashSet<Tag>>,
    tys: HashMap<VarName<'a>, TypeIdx>,
    oprs: HashMap<&'static str, (Type, Type, Type)>,
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn with_predef() -> Self {
        let mut ty_cnt = Counter::default();
        let tys = Predef::types(&mut ty_cnt);
        let vars = HashMap::from_iter(
            Predef::funcs(&tys)
                .into_iter()
                .map(|func| (func.name, func.ty)),
        );
        let oprs = HashMap::from_iter(
            Predef::oprs(&tys)
                .into_iter()
                .map(|opr| (opr.name, (opr.left_ty, opr.right_ty, opr.res_ty))),
        );

        Self {
            ty_cnt,
            ft_cnt: Counter::default(),
            tag_cnt: Counter::default(),
            vars,
            ctors: HashMap::new(),
            tags: HashMap::new(),
            cons: HashMap::new(),
            tys,
            oprs,
        }
    }

    #[inline]
    pub fn next_ft(&mut self) -> Type {
        Type::Free(self.ft_cnt.step())
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
    pub fn pop_var(&mut self, name: VarName<'a>, prev_ty: Option<Type>) {
        match prev_ty {
            Some(prev_ty) => self.push_var(name, prev_ty),
            None => None,
        };
    }

    #[inline]
    pub fn push_vars(&mut self, names: &[VarName<'a>], tys: Vec<Type>) -> Vec<Option<Type>> {
        names
            .iter()
            .zip(tys)
            .map(|(name, ty)| self.push_var(name, ty))
            .collect()
    }

    #[inline]
    pub fn pop_vars(&mut self, names: &[VarName<'a>], prev_tys: Vec<Option<Type>>) {
        names
            .iter()
            .zip(prev_tys)
            .for_each(|(name, prev_ty)| self.pop_var(name, prev_ty));
    }

    #[inline]
    pub fn ctor(&self, name: &Name<'a>) -> VerifyRes<(Type, TypeIdx)> {
        match self.ctors.get(name.val) {
            Some(x) => Ok(x.clone()),
            None => Err(VerifyErr::UnknownCtor(
                name.loc.into_span(),
                name.val.to_string(),
            )),
        }
    }

    #[inline]
    pub fn add_ctor(&mut self, name: &Name<'a>, from: Type, into: TypeIdx) -> VerifyRes<()> {
        match self.ctors.insert(name.val, (from, into)) {
            Some(_) => Err(VerifyErr::CtorRedefinition(
                name.loc.into_span(),
                name.val.to_string(),
            )),
            None => {
                let idx = self.tag_cnt.step();
                self.tags.insert(name.val, idx);

                self.cons
                    .entry(into)
                    .and_modify(|tags| {
                        tags.insert(idx);
                    })
                    .or_insert([idx].into());
                Ok(())
            }
        }
    }

    #[inline]
    pub fn tag(&self, name: &Name<'a>) -> VerifyRes<Tag> {
        match self.tags.get(name.val) {
            Some(tag) => Ok(*tag),
            None => Err(VerifyErr::UnknownName(
                name.loc.into_span(),
                name.val.to_string(),
            )),
        }
    }

    #[inline]
    pub fn ty_cons(&self, idx: TypeIdx) -> &HashSet<Tag> {
        self.cons.get(&idx).unwrap()
    }

    #[inline]
    pub fn ty(&self, name: &Name<'a>) -> VerifyRes<TypeIdx> {
        match self.tys.get(name.val) {
            Some(ty) => Ok(*ty),
            None => Err(VerifyErr::UnknownType(
                name.loc.into_span(),
                name.val.to_string(),
            )),
        }
    }

    #[inline]
    pub fn prim_ty(&self, name: &Name<'a>) -> VerifyRes<Type> {
        self.ty(name).map(Type::Prim)
    }

    #[inline]
    pub fn unit_ty(&self) -> Type {
        Type::Prim(*self.tys.get("Unit").unwrap())
    }

    #[inline]
    pub fn int_ty(&self) -> Type {
        Type::Prim(*self.tys.get("Int").unwrap())
    }

    #[inline]
    pub fn double_ty(&self) -> Type {
        Type::Prim(*self.tys.get("Double").unwrap())
    }

    #[inline]
    pub fn str_ty(&self) -> Type {
        Type::Prim(*self.tys.get("Str").unwrap())
    }

    #[inline]
    pub fn get_prim(&self, idx: TypeIdx) -> VarName<'a> {
        for (name, i) in self.tys.iter() {
            if *i == idx {
                return name;
            }
        }
        unreachable!("It's guaranteed that a type with such index exists")
    }

    #[inline]
    pub fn add_ty(&mut self, name: &Name<'a>) -> VerifyRes<TypeIdx> {
        let idx = self.ty_cnt.step();
        match self.tys.insert(name.val, idx) {
            Some(_) => Err(VerifyErr::TypeRedefinition(
                name.loc.into_span(),
                name.val.to_string(),
            )),
            None => Ok(idx),
        }
    }

    pub fn opr(&self, name: &Name<'a>) -> VerifyRes<(Type, Type, Type)> {
        match self.oprs.get(name.val) {
            Some(x) => Ok(x.clone()),
            None => Err(VerifyErr::UnknownOpr(
                name.loc.into_span(),
                name.val.to_string(),
            )),
        }
    }

    #[inline]
    pub fn apply(&mut self, subst: &Subst) {
        for ty in self.vars.values_mut() {
            *ty = ty.apply(subst);
        }
    }

    #[inline]
    pub fn resolve(&mut self, left: &Type, right: &Type, loc: &Loc<'a>) -> VerifyRes<Subst> {
        match resolve(left, right) {
            Ok(subst) => Ok(subst),
            Err(_) => Err(VerifyErr::TypeMismatch(
                loc.into_span(),
                left.show(self),
                right.show(self),
            )),
        }
    }

    #[inline]
    pub fn resolve_apply(&mut self, left: &Type, right: &Type, loc: &Loc<'a>) -> VerifyRes<Subst> {
        let subst = self.resolve(left, right, loc)?;
        self.apply(&subst);
        Ok(subst)
    }
}

type ResolveRes<T> = Result<T, ()>;

fn resolve(left: &Type, right: &Type) -> ResolveRes<Subst> {
    fn go(subst: &mut Subst, left: &Type, right: &Type) -> ResolveRes<()> {
        match (left, right) {
            (Type::Prim(a), Type::Prim(b)) if a == b => Ok(()),
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
            _ => Err(()),
        }
    }

    let mut subst = Subst::new();
    go(&mut subst, left, right)?;
    Ok(subst)
}

pub trait TypeExt {
    fn show(&self, ctx: &Ctx) -> String;
    fn contains_ft(&self, ft: FreeIdx) -> bool;
    fn split<'a>(
        self,
        tpl: &Tpl<'a>,
        ctx: &mut Ctx<'a>,
    ) -> VerifyRes<(Vec<VarName<'a>>, Vec<Type>)>;
}

impl TypeExt for Type {
    fn show(&self, ctx: &Ctx) -> String {
        match self {
            Type::Prim(idx) => ctx.get_prim(*idx).to_string(),
            Type::Pair(a, b) => format!("({}, {})", a.show(ctx), b.show(ctx)),
            Type::Func(a, b) => format!("{} -> {}", a.show(ctx), b.show(ctx)),
            Type::Free(idx) => format!("<{idx}>"),
        }
    }

    fn contains_ft(&self, ft: FreeIdx) -> bool {
        match self {
            Type::Prim(_) => false,
            Type::Pair(a, b) => a.contains_ft(ft) || b.contains_ft(ft),
            Type::Func(a, b) => a.contains_ft(ft) || b.contains_ft(ft),
            Type::Free(idx) => *idx == ft,
        }
    }

    fn split<'a>(
        self,
        tpl: &Tpl<'a>,
        ctx: &mut Ctx<'a>,
    ) -> VerifyRes<(Vec<VarName<'a>>, Vec<Type>)> {
        #[inline]
        fn add_var<'b>(
            vars: &mut HashSet<VarName<'b>>,
            tys: &mut Vec<Type>,
            var: &Name<'b>,
            ty: Type,
        ) -> VerifyRes<()> {
            tys.push(ty);
            match vars.insert(var.val) {
                true => Ok(()),
                false => Err(VerifyErr::InvalidPattern(
                    var.loc.into_span(),
                    var.to_string(),
                )),
            }
        }

        fn go<'b>(
            vars: &mut HashSet<VarName<'b>>,
            tys: &mut Vec<Type>,
            ctx: &mut Ctx<'b>,
            ty: Type,
            tpl: &Tpl<'b>,
        ) -> VerifyRes<()> {
            match (ty, tpl) {
                (ty, Tpl::Var(name)) => add_var(vars, tys, name, ty),
                (Type::Pair(a, b), Tpl::Pair(tpl)) => {
                    go(vars, tys, ctx, *a, &tpl.left)?;
                    go(vars, tys, ctx, *b, &tpl.right)
                }
                (ty, Tpl::As(tpl)) => {
                    add_var(vars, tys, &tpl.name, ty.clone())?;
                    go(vars, tys, ctx, ty, &tpl.tpl)
                }
                (Type::Free(idx), Tpl::Pair(tpl)) => {
                    let left_ty = ctx.next_ft();
                    let right_ty = ctx.next_ft();
                    ctx.resolve_apply(
                        &Type::Free(idx),
                        &Type::pair(left_ty.clone(), right_ty.clone()),
                        &tpl.loc,
                    )?;
                    go(vars, tys, ctx, left_ty, &tpl.left)?;
                    go(vars, tys, ctx, right_ty, &tpl.right)
                }
                // (_, Tpl::Empty(tpl)) => Ok(()),
                (ty, Tpl::Pair(tpl)) => Err(VerifyErr::PatternNotMatch(
                    tpl.loc.into_span(),
                    ty.show(ctx),
                    "(_, _)".to_string(),
                )),
                (ty, Tpl::Empty(tpl)) => Err(VerifyErr::PatternNotMatch(
                    tpl.loc.into_span(),
                    ty.show(ctx),
                    "()".to_string(),
                )),
            }
        }

        let mut vars = HashSet::new();
        let mut tys = Vec::new();
        go(&mut vars, &mut tys, ctx, self, tpl)?;
        Ok((Vec::from_iter(vars), tys))
    }
}
