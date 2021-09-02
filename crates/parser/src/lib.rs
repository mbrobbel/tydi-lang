use rowan::GreenNode;
use tydi_lang_lexer::Lexer;
use tydi_lang_syntax::SyntaxNode;

mod source;
pub use source::Source;

mod sink;
pub use sink::Sink;

mod event;
pub use event::Event;

mod parser;
pub use parser::Parser;

pub mod grammar;

mod marker;
pub use marker::*;

/// The result of parsing.
pub struct Parse {
    node: GreenNode,
    pub errors: Vec<String>, // todo(mb): error enum
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.node.clone())
    }
}

pub fn parse<T>(input: T) -> Parse
where
    T: AsRef<str>,
{
    let tokens = Lexer::new(input.as_ref()).collect::<Vec<_>>();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn type_def() {
        assert_eq!(
            format!("{:#?}", parse("type Foo = Bar; ").syntax()),
            r#"Root@0..16
  TypeDefinition@0..16
    Type@0..4 "type"
    Whitespace@4..5 " "
    Identifier@5..8 "Foo"
    Whitespace@8..9 " "
    Equals@9..10 "="
    Whitespace@10..11 " "
    Identifier@11..14 "Bar"
    Semicolon@14..15 ";"
    Whitespace@15..16 " "
"#
        );

        assert_eq!(
            format!("{:#?}", parse("type  Foo = Bar").syntax()),
            r#"Root@0..15
  TypeDefinition@0..15
    Type@0..4 "type"
    Whitespace@4..6 "  "
    Identifier@6..9 "Foo"
    Whitespace@9..10 " "
    Equals@10..11 "="
    Whitespace@11..12 " "
    Identifier@12..15 "Bar"
"#
        );
    }
}
