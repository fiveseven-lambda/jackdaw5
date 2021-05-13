pub struct Slicer {
    buffer: String,
    cursor: usize,
    count: usize,
}

impl Slicer {
    pub fn new() -> Slicer {
        Slicer {
            buffer: String::new(),
            cursor: 0,
            count: 0,
        }
    }
    pub fn set(&mut self, s: String) {
        self.buffer = s;
        self.cursor = 0;
        self.count = 0;
    }
    pub fn next(&mut self) -> Option<(char, usize, usize)> {
        let c = self.buffer[self.cursor..].chars().next()?;
        let ret = (c, self.cursor, self.count);
        self.cursor += c.len_utf8();
        self.count += 1;
        Some(ret)
    }
}

#[test]
fn test_slicer() {
    let mut slicer = Slicer::new();
    slicer.set("𠮷野家で𩸽".to_string());
    assert_eq!(slicer.next(), Some('𠮷'));
    assert_eq!(slicer.count, 1);
    assert_eq!(slicer.next(), Some('野'));
    assert_eq!(slicer.count, 2);
    assert_eq!(slicer.next(), Some('家'));
    assert_eq!(slicer.count, 3);
    assert_eq!(slicer.next(), Some('で'));
    assert_eq!(slicer.count, 4);
    assert_eq!(slicer.next(), Some('𩸽'));
    assert_eq!(slicer.count, 5);
}
