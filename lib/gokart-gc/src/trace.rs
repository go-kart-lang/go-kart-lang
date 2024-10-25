use std::any::{type_name, Any};

// Трейт для downcasting
pub trait AnyUpcast {
    fn as_any(&self) -> &(dyn 'static + Any);
    fn type_name(&self) -> &'static str;
}

impl<T: Any> AnyUpcast for T {
    fn as_any(&self) -> &(dyn 'static + Any) {
        self
    }
    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }
}

pub trait Trace: AnyUpcast + 'static {
    fn trace<'a>(&self, vac: &super::Vacuum<'a>);
}
