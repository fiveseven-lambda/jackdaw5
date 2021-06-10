use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Add;

// 文字の位置（何行目，何文字目）を 0-indexed で表す．
// Ord の derive は (line, column) の辞書式順序（メンバの宣言順）
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharPos {
    line: usize,
    column: usize,
}

// 半開区間
#[derive(Clone)]
pub struct Pos {
    start: CharPos,
    end: CharPos,
}

// new
impl CharPos {
    pub fn new(line: usize, column: usize) -> CharPos {
        CharPos { line: line, column: column }
    }
}
impl Pos {
    pub fn new(start: CharPos, end: CharPos) -> Pos {
        debug_assert!((start.line, start.column) <= (end.line, end.column));
        Pos { start: start, end: end }
    }
}

// Display, Debug
impl Display for CharPos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line + 1, self.column + 1)
    }
}
impl Debug for CharPos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{}",
            self.start.line + 1,
            self.start.column + 1,
            self.end.line + 1,
            self.end.column
        )
    }
}
impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{:?}, {:?})", self.start, self.end)
    }
}

// 順序あり足し算
impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other: Pos) -> Pos {
        debug_assert!((self.end.line, self.end.column) <= (other.start.line, other.start.column));
        Pos::new(self.start, other.end)
    }
}
impl Add<Option<Pos>> for Pos {
    type Output = Pos;
    fn add(self, right: Option<Pos>) -> Pos {
        match right {
            Some(right) => self + right,
            None => self,
        }
    }
}
impl Add<Pos> for Option<Pos> {
    type Output = Pos;
    fn add(self, right: Pos) -> Pos {
        match self {
            Some(left) => left + right,
            None => right,
        }
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

#[test]
fn test_add() {
    let left = || Pos::new(CharPos::new(2, 3), CharPos::new(2, 6)); // 2 行目の 3 から 6 文字目
    let right = || Pos::new(CharPos::new(5, 1), CharPos::new(5, 4)); // 5 行目の 1 から 4 文字目

    // 合わせると， 2 行目の 3 文字目から 5 行目の 4 文字目までになる
    assert_eq!((left() + right()).into_inner(), (2, 3)..=(5, 4));
    // 片方が Option でも同じ
    assert_eq!((left() + Some(right())).into_inner(), (2, 3)..=(5, 4));
    assert_eq!((Some(left()) + right()).into_inner(), (2, 3)..=(5, 4));

    // 片方が None なら何も変わらない
    assert_eq!((left() + None).into_inner(), (2, 3)..=(2, 6));
    assert_eq!((None + right()).into_inner(), (5, 1)..=(5, 4));
}
