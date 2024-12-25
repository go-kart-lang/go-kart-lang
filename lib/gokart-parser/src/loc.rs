#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub idx: usize,
}

impl Pos {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loc {
    pub begin: Pos,
    pub end: Pos,
}

impl Loc {
    pub fn new(begin: usize, end: usize) -> Self {
        Self {
            begin: Pos::new(begin),
            end: Pos::new(end),
        }
    }
}
