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
    pub fn ptr(self) -> TypePtr {
        Box::new(self)
    }

    #[inline]
    pub fn func(from: Type, into: Type) -> Type {
        Type::Func(from.ptr(), into.ptr())
    }

    #[inline]
    pub fn pair(from: Type, into: Type) -> Type {
        Type::Pair(from.ptr(), into.ptr())
    }
}

impl Type {
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
