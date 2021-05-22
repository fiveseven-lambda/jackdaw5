pub struct Pos {
    line: usize,
    pos: usize,
}

impl Pos {
    pub fn new(line: usize, pos: usize) -> Pos {
        Pos { line: line, pos: pos }
    }
}

use std::fmt::{self, Debug, Display, Formatter};
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}
impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}
