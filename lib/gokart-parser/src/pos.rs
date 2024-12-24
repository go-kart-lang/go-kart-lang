use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
    pub idx: usize,
}

impl Pos {
    pub fn new(line: usize, col: usize, idx: usize) -> Self {
        Self { line, col, idx }
    }

    pub fn next(&mut self, ch: char) {
        self.idx += 1;
        self.col += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self::new(1, 1, 0)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}
