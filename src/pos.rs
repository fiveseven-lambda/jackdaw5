pub struct Pos {
    line: usize,
    pos: usize,
}

pub trait CharPos: Iterator<Item = (usize, char)> + Sized {
    fn pos(&mut self) -> CharPosIter<Self>;
}

use std::str::CharIndices;

impl<'a> CharPos for CharIndices<'a> {
    fn pos(&mut self) -> CharPosIter<CharIndices<'a>> {
        CharPosIter { line: 1, pos: 1, char_indices: self }
    }
}

pub struct CharPosIter<'a, Iter: Iterator<Item = (usize, char)>> {
    line: usize,
    pos: usize,
    char_indices: &'a mut Iter,
}

impl<'a, Iter: Iterator<Item = (usize, char)>> Iterator for CharPosIter<'a, Iter> {
    type Item = (usize, char, Pos);
    fn next(&mut self) -> Option<Self::Item> {
        let (index, c) = self.char_indices.next()?;
        let ret = (index, c, Pos { line: self.line, pos: self.pos });
        if c == '\n' {
            self.line += 1;
            self.pos = 1;
        } else {
            self.pos += 1;
        }
        Some(ret)
    }
}

use std::fmt;

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}
