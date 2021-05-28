use std::fmt::{self, Debug, Display, Formatter};

// 文字の位置（何行目，何文字目）を表す．
// Ord の derive は (line, column) の辞書式順序（メンバの宣言順）
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct End {
    line: usize,
    column: usize,
}

impl End {
    pub fn new(line: usize, column: usize) -> End {
        End { line: line, column: column }
    }
}

impl Display for End {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Debug for End {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

// 閉区間
#[derive(Clone)]
pub struct Pos {
    start: End,
    end: End,
}

impl Pos {
    pub fn new(start: End, end: End) -> Pos {
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

// 順序あり足し算
impl std::ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, right: Pos) -> Pos {
        Pos::new(self.start, right.end)
    }
}

impl std::ops::Add<Option<Pos>> for Pos {
    type Output = Pos;
    fn add(self, right: Option<Pos>) -> Pos {
        match right {
            Some(right) => self + right,
            None => self,
        }
    }
}

impl std::ops::Add<Pos> for Option<Pos> {
    type Output = Pos;
    fn add(self, right: Pos) -> Pos {
        match self {
            Some(left) => left + right,
            None => right,
        }
    }
}

#[cfg(test)]
impl End {
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
    let left = || Pos::new(End::new(2, 3), End::new(2, 6)); // 2 行目の 3 から 6 文字目
    let right = || Pos::new(End::new(5, 1), End::new(5, 4)); // 5 行目の 1 から 4 文字目

    // 合わせると， 2 行目の 3 文字目から 5 行目の 4 文字目までになる
    assert_eq!((left() + right()).into_inner(), (2, 3)..=(5, 4));
    // 片方が Option でも同じ
    assert_eq!((left() + Some(right())).into_inner(), (2, 3)..=(5, 4));
    assert_eq!((Some(left()) + right()).into_inner(), (2, 3)..=(5, 4));

    // 片方が None なら何も変わらない
    assert_eq!((left() + None).into_inner(), (2, 3)..=(2, 6));
    assert_eq!((None + right()).into_inner(), (5, 1)..=(5, 4));
}
