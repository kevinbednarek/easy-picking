#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Kind {
    Identifier,
    Number,
    String,
    If,
    While,
    For,
    #[default]
    Eof, //End of file
    Plus,
}