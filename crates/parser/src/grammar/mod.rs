use crate::{CompletedMarker, Parser};
use tydi_lang_syntax::SyntaxKind;

pub mod statement;

pub fn root(p: &mut Parser) -> CompletedMarker {
    let mut marker = p.start();

    while !p.at_end() {
        statement::statement(p);
    }

    marker.complete(p, SyntaxKind::Root)
}
