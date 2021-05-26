use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharPos {
    line: usize,
    column: usize,
}

impl CharPos {
    pub fn new(line: usize, column: usize) -> CharPos {
        CharPos { line: line, column: column }
    }
}

impl Display for CharPos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Debug for CharPos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Clone)]
pub struct Pos {
    start: CharPos,
    end: CharPos,
}

impl Pos {
    pub fn new(start: CharPos, end: CharPos) -> Pos {
        Pos { start: start, end: end }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[cfg(test)]
impl CharPos {
    pub fn into_inner(self) -> (usize, usize) {
        (self.line, self.column)
    }
}
#[cfg(test)]
impl Pos {
    pub fn into_inner(self) -> std::ops::RangeInclusive<(usize, usize)> {
        self.start.into_inner()..=self.end.into_inner()
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        Pos::new(self.start.min(other.start), self.end.max(other.end))
    }
}
impl std::ops::Add<Option<Pos>> for Pos {
    type Output = Pos;
    fn add(self, other: Option<Pos>) -> Pos {
        match other {
            Some(other) => self + other,
            None => self,
        }
    }
}
