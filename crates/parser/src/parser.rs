use crate::{grammar::root, Event, Marker, Source};
use tydi_lang_lexer::TokenKind;
use tydi_lang_syntax::SyntaxKind;

pub struct Parser<'tok, 'src> {
    source: Source<'tok, 'src>,
    pub events: Vec<Event>,
}

impl<'tok, 'src> Parser<'tok, 'src> {
    pub fn new(source: Source<'tok, 'src>) -> Self {
        Self {
            source,
            events: Vec::default(),
        }
    }
}

impl Parser<'_, '_> {
    /// Returns a new Marker at the current position. Puts a Placeholder event.
    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }

    pub fn bump(&mut self) {
        self.source.next().unwrap();
        self.events.push(Event::AddToken);
    }

    pub fn error(&mut self) {
        // todo(mb): enum and location
        self.events.push(Event::Error("syntax error".to_string()));
        if !self.at_end() {
            let mut marker = self.start();
            self.bump();
            marker.complete(self, SyntaxKind::Error);
        }
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump()
        } else {
            self.error()
        }
    }

    pub fn at(&mut self, kind: TokenKind) -> bool {
        self.peek() == Some(kind)
    }

    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek_kind()
    }
}

impl Parser<'_, '_> {
    pub fn parse(mut self) -> Vec<Event> {
        root(&mut self);
        self.events
    }
}
