use crate::{CompletedMarker, Parser};
use tydi_lang_lexer::TokenKind;
use tydi_lang_syntax::SyntaxKind;

pub fn statement(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::Type) {
        Some(type_definition(p))
    } else {
        p.error();
        None
    }
}

pub fn type_definition(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::Type));
    let mut marker = p.start();
    p.bump();

    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::Equals);
    p.expect(TokenKind::Identifier);
    p.expect(TokenKind::Semicolon);

    marker.complete(p, SyntaxKind::TypeDefinition)
}
