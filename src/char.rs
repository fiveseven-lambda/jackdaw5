pub struct Pos {
    line: usize,
    pos: usize,
}

pub struct CharPos {
    value: char,
    pos: Pos,
}

pub fn read(path: &std::path::PathBuf) -> Result<Vec<CharPos>, std::io::Error> {
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    let mut ret = Vec::new();
    let mut reader = BufReader::new(File::open(path)?);
    for line in 0.. {
        let mut buf = String::new();
        if reader.read_line(&mut buf)? == 0 {
            break;
        }
        for (pos, c) in buf.chars().enumerate() {
            ret.push(CharPos{value: c, pos: Pos {line: line + 1, pos: pos + 1}});
        }
    }
    Ok(ret)
}

use std::fmt;

impl fmt::Display for CharPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}

impl fmt::Binary for CharPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pos)
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Debug for CharPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{0}` ({0:b})", self)
    }
}