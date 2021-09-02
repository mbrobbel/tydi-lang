use crate::{Event, Parse};
use rowan::{GreenNodeBuilder, Language};
use std::mem;
use tydi_lang_lexer::Token;
use tydi_lang_syntax::TydiLang;

pub struct Sink<'tok, 'src> {
    builder: GreenNodeBuilder<'static>,
    tokens: &'tok [Token<'src>],
    cursor: usize,
    events: Vec<Event>,
    errors: Vec<String>, // todo(mb): enum
}

impl<'tok, 'src> Sink<'tok, 'src> {
    pub fn new(tokens: &'tok [Token<'src>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::default(),
            tokens,
            cursor: 0,
            events,
            errors: Vec::default(),
        }
    }
}

impl Sink<'_, '_> {
    fn token(&mut self) {
        let Token { kind, slice } = self.tokens[self.cursor];

        self.builder
            .token(TydiLang::kind_to_raw(kind.into()), slice);

        self.cursor += 1;
    }

    fn skip_trivia(&mut self) {
        while let Some(Token { kind, .. }) = self.tokens.get(self.cursor) {
            if !kind.is_trivia() {
                break;
            }

            self.token();
        }
    }

    pub fn finish(mut self) -> Parse {
        (0..self.events.len()).for_each(|idx| {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    let mut kinds = vec![kind];

                    let mut idx = idx;
                    let mut forward_parent = forward_parent;

                    while let Some(fp) = forward_parent {
                        idx += fp;

                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } =
                            mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(TydiLang::kind_to_raw(kind));
                    }
                }
                Event::AddToken => self.token(),
                Event::FinishNode => self.builder.finish_node(),
                Event::Error(error) => self.errors.push(error),
                Event::Placeholder => {}
            }

            self.skip_trivia();
        });

        Parse {
            node: self.builder.finish(),
            errors: self.errors,
        }
    }
}
