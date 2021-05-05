pub struct Slicer {
    buffer: String,
    cursor: usize,
}

impl Slicer {
    pub fn from(s: String) -> Slicer {
        Slicer {
            buffer: s,
            cursor: 0,
        }
    }
    pub fn pos(&self) -> usize {
        self.cursor
    }
    pub fn rem(&self) -> &str {
        &self.buffer[self.cursor..]
    }
    pub fn slice(&mut self, from: Option<usize>, to: Option<usize>) -> &str {
        let from = match from {
            Some(index) => self.cursor + index,
            None => self.cursor
        };
        let to = match to {
            Some(index) => self.cursor + index,
            None => self.buffer.len(),
        };
        eprintln!("{} - {}", from, to);
        self.cursor = to;
        &self.buffer[from..to]
    }
}
