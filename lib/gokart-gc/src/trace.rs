use gokart_core::GRef;

use crate::vacuum::Vacuum;

pub trait Trace<R>
where
    R: GRef,
{
    fn trace(&self, vac: &mut Vacuum<R>);
}
