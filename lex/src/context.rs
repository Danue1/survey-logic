pub struct Context<'lex> {
    source: &'lex str,
    index: usize,
}

impl<'lex> Context<'lex> {
    pub const fn new(source: &'lex str) -> Self {
        Self { source, index: 0 }
    }

    pub const fn index(&self) -> usize {
        self.index
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.index..).and_then(|s| s.chars().next())
    }

    pub fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.advance();
        Some(c)
    }

    #[inline]
    pub fn advance(&mut self) {
        self.index += 1;
    }

    #[inline]
    pub fn slice(&self, start: usize) -> String {
        self.source[start..self.index].to_owned()
    }
}
