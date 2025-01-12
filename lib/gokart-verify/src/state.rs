use crate::ty::TypeVar;

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
    pub fn next(&mut self) -> usize {
        self.val += 1;
        self.val
    }
}

#[derive(Debug)]
pub struct State {
    tv_cnt: Counter,
}

impl State {
    #[inline]
    pub fn new() -> Self {
        Self {
            tv_cnt: Counter::new(),
        }
    }

    #[inline]
    pub fn next_tv(&mut self) -> TypeVar {
        self.tv_cnt.next()
    }

    #[inline]
    pub fn next_tvs(&mut self, n: usize) -> Vec<TypeVar> {
        (0..n).map(|_| self.tv_cnt.next()).collect()
    }
}
