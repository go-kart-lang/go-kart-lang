use gokart_core::GRef;

use crate::marker::Marker;

pub trait Trace<R>
where
    R: GRef,
{
    fn trace<C>(&self, vac: C)
    where
        C: Marker<R>;
}
