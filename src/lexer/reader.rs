pub struct Reader<Read> {
    inner: Read,
    prompt: bool,
}

impl<Read> Reader<Read> {
    pub fn new(inner: Read, prompt: bool) -> Reader<Read> {
        Reader {
            inner: inner,
            prompt: prompt,
        }
    }
}

impl<Read: std::io::BufRead> Reader<Read> {
    pub fn read_line(&mut self) -> Result<Option<String>, std::io::Error> {
        if self.prompt {
            print!("> ");
            use std::io::Write;
            std::io::stdout().flush()?;
        }
        let mut s = String::new();
        Ok((self.inner.read_line(&mut s)? > 0).then(|| s))
    }
}
