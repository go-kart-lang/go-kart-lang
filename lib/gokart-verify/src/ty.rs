pub type TypeVar = usize;
pub type TypeName = &'static str;

#[derive(Debug, Clone)]
pub enum Type {
    Var(TypeVar),
    Func(TypePtr, TypePtr),
    Prim(TypeName, Vec<Type>),
}

pub type TypePtr = Box<Type>;
impl Type {
    pub fn ptr(self) -> TypePtr {
        Box::new(self)
    }
}
