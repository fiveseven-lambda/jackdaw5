pub struct Reader<R> {
    inner: R,
    interactive: bool,
    line: usize,
}

use std::io::BufRead;

impl<R> Reader<R> {
    pub fn new(inner: R, interactive: bool) -> Reader<R> {
        Reader {
            inner: inner,
            interactive: interactive,
            line: 0,
        }
    }
    pub fn line(&self) -> usize {
        self.line
    }
}

impl<R: BufRead> Reader<R> {
    pub fn read_line(&mut self) -> Result<(usize, String), std::io::Error> {
        if self.interactive {
            print!("> ");
            std::io::Write::flush(&mut std::io::stdout())?;
        }
        self.line += 1;
        let mut s = String::new();
        Ok((self.inner.read_line(&mut s)?, s))
    }
}
