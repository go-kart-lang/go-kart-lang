use crate::err::{VerifyErr, VerifyRes};
use gokart_core::{LocExt, Name, Type, TypeIdx, VarName};
use std::collections::HashMap;

#[derive(Debug)]
struct Counter {
    val: usize,
}

impl Counter {
    #[inline]
    pub fn new() -> Self {
        Self { val: 0 }
    }

    #[inline]
    pub fn start_with(val: usize) -> Self {
        Self { val }
    }

    #[inline]
    pub fn next(&mut self) -> usize {
        self.val += 1;
        self.val
    }
}

#[derive(Debug)]
pub struct State<'a> {
    ty_cnt: Counter,
    ft_cnt: Counter,
    ctors: HashMap<VarName<'a>, (Type, Type)>,
    tys: HashMap<VarName<'a>, TypeIdx>,
}

impl<'a> State<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            ft_cnt: Counter::new(),
            ty_cnt: Counter::start_with(5), // todo
            ctors: HashMap::new(),
            tys: HashMap::from([
                ("Unit", Type::unit_idx()),
                ("Int", Type::int_idx()),
                ("Double", Type::double_idx()),
                ("Str", Type::str_idx()),
            ]),
        }
    }

    #[inline]
    pub fn next_ft(&mut self) -> Type {
        Type::Free(self.ft_cnt.next())
    }

    #[inline]
    pub fn ctor(&self, name: &Name<'a>) -> VerifyRes<&(Type, Type)> {
        match self.ctors.get(name.val) {
            Some(x) => Ok(x),
            None => Err(VerifyErr::UnknownCtor(
                name.loc.into_span(),
                name.val.to_string(),
            )),
        }
    }

    #[inline]
    pub fn add_ctor(&mut self, name: &Name<'a>, from: Type, into: Type) -> VerifyRes<()> {
        match self.ctors.insert(name.val, (from, into)) {
            Some(_) => Err(VerifyErr::CtorRedefinition(
                name.loc.into_span(),
                name.val.to_string(),
            )),
            None => Ok(()),
        }
    }

    #[inline]
    pub fn ty(&self, name: &Name<'a>) -> VerifyRes<Type> {
        match self.tys.get(name.val) {
            Some(ty) => Ok(Type::Prim(*ty)),
            None => Err(VerifyErr::UnknownType(
                name.loc.into_span(),
                name.val.to_string(),
            )),
        }
    }

    #[inline]
    pub fn add_ty(&mut self, name: &Name<'a>) -> VerifyRes<Type> {
        let idx = self.ty_cnt.next();
        match self.tys.insert(name.val, idx) {
            Some(_) => Err(VerifyErr::TypeRedefinition(
                name.loc.into_span(),
                name.val.to_string(),
            )),
            None => Ok(Type::Prim(idx)),
        }
    }
}
