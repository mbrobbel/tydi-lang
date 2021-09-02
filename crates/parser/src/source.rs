use tydi_lang_lexer::{Token, TokenKind};

pub struct Source<'tok, 'src> {
    tokens: &'tok [Token<'src>],
    cursor: usize,
}

impl<'tok, 'src> Source<'tok, 'src> {
    pub fn new(tokens: &'tok [Token<'src>]) -> Self {
        Self { tokens, cursor: 0 }
    }
}

impl Source<'_, '_> {
    pub fn next(&mut self) -> Option<&Token> {
        self.skip_trivia();
        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;
        Some(token)
    }

    pub fn peek_kind(&mut self) -> Option<TokenKind> {
        self.skip_trivia();
        self.peek_kind_raw()
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        self.skip_trivia();
        self.peek_token_raw()
    }

    fn skip_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, TokenKind::is_trivia)
    }

    fn peek_kind_raw(&self) -> Option<TokenKind> {
        self.peek_token_raw().map(|Token { kind, .. }| *kind)
    }

    fn peek_token_raw(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
}
