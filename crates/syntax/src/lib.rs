use std::mem;
use tydi_lang_lexer::TokenKind;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum SyntaxKind {
    // Safety:
    // - The variants in this enum **must** extend TokenKind from lexer.
    Type,
    Equals,
    Semicolon,
    Identifier,
    Whitespace,
    Comment,
    Error,

    // Safety:
    // - This **must** be the last variant of this enum.
    // - The transmute on rowan::Language::kind_from_raw depends on it.
    TypeDefinition,
    Root,
}

impl From<TokenKind> for SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        // Safety:
        // - SyntaxKind is always an extension of TokenKind.
        unsafe { mem::transmute::<u16, SyntaxKind>(kind as u16) }
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TydiLang {}

impl rowan::Language for TydiLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::Root as u16);
        // Safety:
        // - Root variant is always the last variant of the SyntaxKind enum.
        unsafe { mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxToken = rowan::SyntaxToken<TydiLang>;
pub type SyntaxNode = rowan::SyntaxNode<TydiLang>;
pub type SyntaxElement = rowan::SyntaxElement<TydiLang>;
