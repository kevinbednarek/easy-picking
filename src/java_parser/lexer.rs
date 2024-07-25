use crate::java_parser::cursor::Cursor;
use crate::java_parser::error::LexicalError;
use crate::java_parser::indentation::Indentation;
use crate::java_parser::token::{TokenFlags, TokenKind, TokenValue};

pub struct Lexer<'src> {
    /// Source code to be lexed
    source: &'src str,
    /// A pointer to the current character of the source code which is being lexed
    cursor: Cursor<'src>,
    /// The kind of current token
    current_kind: TokenKind,
    /// The range of the current token
    current_range: TextRange,
    /// The value of the current token
    current_value: TokenValue,
    /// Flags for the current token
    current_flags: TokenFlags,
    state: State,
    nesting: u32,
    indentations: Vec<Indentation>,
    pending_indentation: Option<Indentation>,
    errors: Vec<LexicalError>,
}
enum State {
    AfterNewLine,
    NonEmptyLogicalLine,
    AfterEqual,
    Other,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl Lexer {
    pub fn new() {

    }

    pub fn read_char(&self) {

    }

    pub fn next_token(&self) {

    }
}