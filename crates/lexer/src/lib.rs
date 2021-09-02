use logos::Logos;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Logos)]
#[repr(u16)]
pub enum TokenKind {
    // Keywords
    #[token("type")]
    Type,

    // Punctuation
    #[token("=")]
    Equals,
    #[token(";")]
    Semicolon,

    // Literals
    #[regex("[a-zA-Z][a-zA-Z_]*")]
    Identifier,

    // Trivia
    #[regex(r"[\p{White_Space}]+")]
    Whitespace,

    #[regex(r"//[^\n]*")]
    Comment,

    #[error]
    Error,
}

impl TokenKind {
    pub fn is_literal(self) -> bool {
        match self {
            TokenKind::Identifier => true,
            _ => false,
        }
    }

    pub fn is_trivia(self) -> bool {
        match self {
            TokenKind::Comment | TokenKind::Error | TokenKind::Whitespace => true,
            _ => false,
        }
    }
}

pub struct Lexer<'src>(logos::Lexer<'src, TokenKind>);

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self(TokenKind::lexer(input))
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.0.next()?;
        let slice = self.0.slice();
        Some(Token { kind, slice })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub slice: &'src str,
}
