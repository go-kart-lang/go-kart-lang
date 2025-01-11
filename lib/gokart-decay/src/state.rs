use gokart_core::{Tag, Var};

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
    var_cnt: Counter,
    tag_cnt: Counter,
}

impl State {
    #[inline]
    pub fn new() -> Self {
        Self {
            var_cnt: Counter::new(),
            tag_cnt: Counter::new(),
        }
    }

    #[inline]
    pub fn next_var(&mut self) -> Var {
        self.var_cnt.next()
    }

    #[inline]
    pub fn next_tag(&mut self) -> Tag {
        self.tag_cnt.next()
    }
}
