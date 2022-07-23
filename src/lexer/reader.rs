pub struct Reader<'a> {
    source: &'a [u8],
    offset: usize,
    line: usize,
}

impl<'a> Reader<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self {
            source,
            offset: 0,
            line: 0,
        }
    }

    pub fn peek(&self) -> Option<u8> {
        self.source.get(self.offset).cloned()
    }

    pub fn skip(&mut self) {
        self.offset += 1;
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn get_slice(&self, range: std::ops::Range<usize>) -> &[u8] {
        &self.source[range]
    }

    pub fn next_line(&mut self) {
        self.line += 1;
    }

    pub fn line(&self) -> usize {
        self.line
    }
}
