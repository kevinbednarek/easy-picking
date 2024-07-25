use crate::javascript_parser::kind::Kind;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Token {
    /// Token Type
    pub kind: Kind,
    /// Start offset in source
    pub start: usize,
    /// End offset in source
    pub end: usize,
    pub value: TokenValue,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TokenValue {
    #[default]
    None,
    Number(f64),
    Static,
    //String(Atom<Static>), //TODO: Maybe try this later? Big pain in the ass with the static keyword. Should make this atomic for performance reasons
    String,
}