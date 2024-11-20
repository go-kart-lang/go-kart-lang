use gokart_core::GRef;

pub trait Marker<R>
where
    R: GRef,
{
    fn mark(self, r: R);
}
