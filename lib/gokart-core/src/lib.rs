use std::{hash::Hash, marker::PhantomData, ops::Index};

pub trait GLabel {
    fn next(self) -> Self; // todo: by ref?
}

// todo: move
// impl GLabel for usize {
//     #[inline]
//     fn next(self) -> Self {
//         self + 1
//     }
// }
// impl GRef for usize {}

pub trait GRef: Eq + Hash + Copy {}

pub trait GValue<L>
where
    L: GLabel,
{
}

pub trait GHeap<R, V, L>: Index<R, Output = V>
where
    R: GRef,
    V: GValue<L>,
    L: GLabel,
{
    fn alloc(&mut self, val: V) -> R;
    fn size(&self) -> usize;
    fn index(&self, r: R) -> &V;
}

pub struct VMState<H, R, V, L>
where
    H: GHeap<R, V, L>,
    R: GRef,
    V: GValue<L>,
    L: GLabel,
{
    pub ip: Option<L>,
    pub heap: H,
    pub env: R,
    pub stack: Vec<R>,
    _phantom: PhantomData<V>,
}

pub trait GOpr<L>
where
    L: GLabel,
{
    fn execute<H, R, V>(state: &mut VMState<H, R, V, L>)
    where
        H: GHeap<R, V, L>,
        R: GRef,
        V: GValue<L>; // todo
}

pub trait GGC<H, R, V, L>
where
    H: GHeap<R, V, L>,
    R: GRef,
    V: GValue<L>,
    L: GLabel,
{
    fn is_necessary(&self, state: &VMState<H, R, V, L>) -> bool;
    fn cleanup(&self, state: &mut VMState<H, R, V, L>);
}
