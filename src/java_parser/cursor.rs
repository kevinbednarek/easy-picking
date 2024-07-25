use std::str::Chars;

pub const EOF_CHAR: char = '\0';

#[derive(Clone, Debug)]
pub struct Cursor<'src> {
    pub chars: Chars<'src>,
    pub source_length: usize,  //TODO: maybe u32?
    #[cfg_attr(debug_assertions)]
    pub prev_char: char,
}

impl<'src> Cursor<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
            source_length: source.len(),
            prev_char: EOF_CHAR,
        }
    }
}