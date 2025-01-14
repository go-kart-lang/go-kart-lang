use std::collections::HashMap;

pub type TypeIdx = usize;
pub type FreeIdx = usize;

#[derive(Debug, Clone)]
pub enum Type {
    Prim(TypeIdx),
    Pair(TypePtr, TypePtr),
    Func(TypePtr, TypePtr),
    Free(FreeIdx),
}

impl Type {
    #[inline]
    pub fn unit_idx() -> TypeIdx {
        0
    }

    #[inline]
    pub fn int_idx() -> TypeIdx {
        1
    }

    #[inline]
    pub fn double_idx() -> TypeIdx {
        2
    }

    #[inline]
    pub fn str_idx() -> TypeIdx {
        3
    }

    #[inline]
    pub fn unit() -> Self {
        Self::Prim(Type::unit_idx())
    }

    #[inline]
    pub fn int() -> Self {
        Self::Prim(Type::int_idx())
    }

    #[inline]
    pub fn double() -> Self {
        Self::Prim(Type::double_idx())
    }

    #[inline]
    pub fn str() -> Self {
        Self::Prim(Type::str_idx())
    }

    #[inline]
    pub fn ptr(self) -> TypePtr {
        Box::new(self)
    }

    pub fn apply(&self, subst: &Subst) -> Type {
        match self {
            Type::Prim(idx) => Type::Prim(*idx),
            Type::Pair(left, right) => {
                Type::Pair(left.apply(subst).ptr(), right.apply(subst).ptr())
            }
            Type::Func(from, into) => Type::Func(from.apply(subst).ptr(), into.apply(subst).ptr()),
            Type::Free(idx) => match subst.get(idx) {
                Some(ty) => ty.clone(),
                None => Type::Free(*idx),
            },
        }
    }
}

pub type TypePtr = Box<Type>;

pub type Subst = HashMap<FreeIdx, Type>;
