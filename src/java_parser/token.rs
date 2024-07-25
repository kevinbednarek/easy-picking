use std::cmp::{PartialOrd};



pub struct TokenFlags {
    //TODO
}


pub enum TokenValue {
    None,
    Name(String),
    Int(i64),
    Float(f64),
    String(Box<str>),


}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum TokenKind {
    /// Token kind for a name, commonly known as an identifier
    Name,
    /// Token kind for a primitive integer
    Int,
    /// Token kind for an integer object
    IntObject,
    /// Token kind for a boolean primitive
    Boolean,
    /// Token kind for a boolean object
    BooleanObject,
    /// Token kind for a primitive double
    Double,
    /// Token kind for a double object
    DoubleObject,
    /// Token kind for a primitive long
    Long,
    /// Token kind for a long object
    LongObject,
    /// Token kind for a primitive char
    Char,
    /// Token kind for a character object
    CharObject,
    /// Token kind for a primitive byte
    Byte,
    /// Token kind for a byte object
    ByteObject,
    /// Token kind for a primitive short
    Short,
    /// Token kind for a short object
    ShortObject,
    /// Token kind for a primitive float
    Float,
    /// Token kind for a float object
    FloatObject,
    /// Token kind for a String
    String,
    /// Token kind for a single line comment
    Comment,
    /// Token kind for the start of a multi-line comment
    MultiLineCommentStart,
    /// Token kind for the end of a multi-line comment
    MultiLineCommentEnd,
    /// Token kind for a newline
    NewLine,
    /// Token kind for an indent
    Indent,
    EndOfFile,
    /// Token kind for a question mark `?`.
    Question,
    /// Token kind for an exclamation mark `!`.
    Exclamation,
    /// Token kind for a left parenthesis `(`.
    Lpar,
    /// Token kind for a right parenthesis `)`.
    Rpar,
    /// Token kind for a left square bracket `[`.
    Lsqb,
    /// Token kind for a right square bracket `]`.
    Rsqb,
    /// Token kind for a colon `:`.
    Colon,
    /// Token kind for a comma `,`.
    Comma,
    /// Token kind for a semicolon `;`.
    Semi,
    /// Token kind for plus `+`.
    Plus,
    /// Token kind for minus `-`.
    Minus,
    /// Token kind for star `*`.
    Star,
    /// Token kind for slash `/`.
    Slash,
    /// Token kind for vertical bar `|`.
    Vbar,
    /// Token kind for ampersand `&`.
    Amper,
    /// Token kind for less than `<`.
    Less,
    /// Token kind for greater than `>`.
    Greater,
    /// Token kind for equal `=`.
    Equal,
    /// Token kind for dot `.`.
    Dot,
    /// Token kind for percent `%`.
    Percent,
    /// Token kind for left bracket `{`.
    Lbrace,
    /// Token kind for right bracket `}`.
    Rbrace,
    /// Token kind for double equal `==`.
    EqEqual,
    /// Token kind for not equal `!=`.
    NotEqual,
    /// Token kind for less than or equal `<=`.
    LessEqual,
    /// Token kind for greater than or equal `>=`.
    GreaterEqual,
    /// Token kind for tilde `~`.
    Tilde,
    /// Token kind for caret `^`.
    Caret,
    /// Token kind for left shift `<<`.
    LeftShift,
    /// Token kind for right shift `>>`.
    RightShift,
    /// Token kind for plus equal `+=`.
    PlusEqual,
    /// Token kind for minus equal `-=`.
    MinusEqual,
    /// Token kind for star equal `*=`.
    StarEqual,
    /// Token kind for slash equal `/=`.
    SlashEqual,
    /// Token kind for percent equal `%=`.
    PercentEqual,
    /// Token kind for at `@`.
    At,
    /// Token kind for arrow `->`.
    Rarrow,
    /// Token kind for ellipsis `...`.
    Ellipsis,

    // Reserved/Keywords
    Abstract,
    Assert,
    Break,
    Case,
    Catch,
    Class,
    Continue,
    Default,
    Do,
    Else,
    Enum,
    Extends,
    Final,
    Finally,
    For,
    If,
    Implements,
    Import,
    Instanceof,
    Interface,
    Native,
    New,
    Package,
    Private,
    Protected,
    Public,
    Return,
    Static,
    Strictfp,
    Super,
    Switch,
    Synchronized,
    This,
    Throw,
    Throws,
    Transient,
    Try,
    Void,
    Volatile,
    While,

    Unknown,
}


impl TokenKind {
    #[inline]
    pub const fn is_eof(self) -> bool {
        matches!(self, TokenKind::EndOfFile)
    }

    pub fn is_keyword(self) -> bool {
        TokenKind::Abstract <= self && self <= TokenKind::While
    }

    pub fn is_newline(self) -> bool {
        matches!(self, TokenKind::NewLine)
    }

    pub const fn is_operator(self) -> bool {
        matches!(
            self,
            TokenKind::Lpar
                | TokenKind::Rpar
                | TokenKind::Lsqb
                | TokenKind::Rsqb
                | TokenKind::Comma
                | TokenKind::Semi
                | TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Vbar
                | TokenKind::Amper
                | TokenKind::Less
                | TokenKind::Greater
                | TokenKind::Equal
                | TokenKind::Dot
                | TokenKind::Percent
                | TokenKind::Lbrace
                | TokenKind::Rbrace
                | TokenKind::EqEqual
                | TokenKind::NotEqual
                | TokenKind::LessEqual
                | TokenKind::GreaterEqual
                | TokenKind::Tilde
                | TokenKind::Caret
                | TokenKind::LeftShift
                | TokenKind::RightShift
                | TokenKind::PlusEqual
                | TokenKind::MinusEqual
                | TokenKind::StarEqual
                | TokenKind::SlashEqual
                | TokenKind::PercentEqual
                | TokenKind::At
                | TokenKind::Rarrow
                | TokenKind::Ellipsis
                | TokenKind::Colon
        )
    }
}