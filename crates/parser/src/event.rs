use tydi_lang_syntax::SyntaxKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    StartNode {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    AddToken,
    FinishNode,
    Error(String), // todo(mb): enum + copy
    Placeholder,
}
