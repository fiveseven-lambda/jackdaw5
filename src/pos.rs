use std::fmt::{self, Debug, Display, Formatter};

pub struct Pos {
    line: usize,
    pos: usize,
}

impl Pos {
    pub fn new(line: usize, pos: usize) -> Pos {
        Pos { line: line, pos: pos }
    }
}

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

pub struct Range {
    start: Pos,
    end: Pos,
}

impl Range {
    pub fn new(start: Pos, end: Pos) -> Range {
        Range { start: start, end: end }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[cfg(test)]
impl Pos {
    pub fn into_inner(self) -> (usize, usize) {
        (self.line, self.pos)
    }
}
#[cfg(test)]
impl Range {
    pub fn into_inner(self) -> std::ops::RangeInclusive<(usize, usize)> {
        self.start.into_inner()..=self.end.into_inner()
    }
}