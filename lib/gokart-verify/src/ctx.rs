use crate::err::{VerifyErr, VerifyRes};
use gokart_core::{LocExt, Name, Type, TypeVar, VarName};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TypeScheme {
    pub tvs: Vec<TypeVar>,
    pub ty: Type,
}

impl TypeScheme {
    pub fn empty(ty: Type) -> Self {
        Self {
            tvs: Vec::new(),
            ty,
        }
    }
}

#[derive(Debug)]
pub struct Subst {
    items: HashMap<TypeVar, Type>,
}

impl Subst {
    pub fn identity() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn ty(&self, ty: Type) -> Type {
        match ty {
            Type::Var(tv) => match self.items.get(&tv) {
                Some(found) => found.clone(),
                None => ty.clone(),
            },
            Type::Func(a, b) => Type::Func(self.ty(*a).ptr(), self.ty(*b).ptr()),
            Type::Prim(name, tys) => {
                Type::Prim(name, tys.into_iter().map(|t| self.ty(t)).collect())
            }
        }
    }
}

impl FromIterator<(TypeVar, Type)> for Subst {
    fn from_iter<T: IntoIterator<Item = (TypeVar, Type)>>(iter: T) -> Self {
        Self {
            items: HashMap::from_iter(iter),
        }
    }
}

#[derive(Debug)]
pub struct TypeInfo {
    pub subst: Subst,
    pub ty: Type,
}

impl TypeInfo {
    pub fn new(subst: Subst, ty: Type) -> Self {
        Self { subst, ty }
    }
}

#[derive(Debug)]
pub struct Ctx<'a> {
    tys: HashMap<VarName<'a>, TypeScheme>,
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            tys: HashMap::new(),
        }
    }

    pub fn lookup(&self, name: &Name<'a>) -> VerifyRes<'a, &TypeScheme> {
        match self.tys.get(name.val) {
            Some(val) => Ok(val),
            None => Err(VerifyErr::UnknownName(
                name.loc.into_span(),
                name.val.into(),
            )),
        }
    }
}
