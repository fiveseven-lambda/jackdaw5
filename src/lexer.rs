mod reader;
mod slicer;

pub struct Lexer<Read> {
    reader: reader::Reader<Read>,
    slicer: slicer::Slicer,
}

impl<Read> Lexer<Read> {
    pub fn new(reader: Read, prompt: bool) -> Lexer<Read> {
        Lexer {
            reader: reader::Reader::new(reader, prompt),
            slicer: slicer::Slicer::new(),
        }
    }
}

impl<Read: std::io::BufRead> Iterator for Lexer<Read> {
    type Item = Result<char, Box<dyn std::error::Error>>;
    fn next(&mut self) -> Option<Self::Item> {
        let (first, start, pos) = loop {
            match self.slicer.next() {
                Some((c, i, j)) => {
                    if !c.is_ascii_whitespace() {
                        break (c, i, j);
                    }
                }
                None => match self.reader.read_line() {
                    Ok(s) => self.slicer.set(s?),
                    Err(err) => return Some(Err(err.into())),
                },
            }
        };
        Some(Ok(first))
    }
}

impl<Read: std::io::BufRead> Lexer<Read> {
    fn comment_out(&mut self) {
    }
}