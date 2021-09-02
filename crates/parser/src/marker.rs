use crate::{Event, Parser};
use tydi_lang_syntax::SyntaxKind;

#[derive(Debug)]
pub struct Marker {
    pos: usize,
    completed: bool,
}

impl Marker {
    pub fn new(pos: usize) -> Self {
        Self {
            pos,
            completed: false,
        }
    }

    pub fn complete(&mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.completed = true;

        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode {
            kind,
            forward_parent: None,
        };

        p.events.push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

impl Drop for Marker {
    fn drop(&mut self) {
        if !self.completed {
            panic!("Marker must be completed")
        }
    }
}

pub struct CompletedMarker {
    pub pos: usize,
}

impl CompletedMarker {
    pub fn precede(self, p: &mut Parser) -> Marker {
        let marker = p.start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = p.events[self.pos]
        {
            *forward_parent = Some(marker.pos - self.pos);
        } else {
            unreachable!();
        }

        marker
    }
}
