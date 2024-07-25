use std::str::Chars;
use crate::javascript_parser::kind::Kind;
use crate::javascript_parser::token::{Token, TokenValue};

pub struct Lexer<'a> {
    /// Source text
    source: &'a str,
    /// The remaining characters
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }

    fn read_next_kind(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            match c {
                '+' => return Kind::Plus,
                _ => {},
            }
        }
        Kind::Eof
    }

    pub fn read_next_token(&mut self) -> Token {
        //TODO: Make this return a result to handle unexpected chars
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token { kind, start, end, value: TokenValue::None } //TODO: Need to fix token value
    }

    /// Get the length of offset from the source text, in UTF-8 bytes
    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn match_keyword(&self, ident: &str) -> Kind {
        //All keywords are 1 <= length <= 10
        if ident.len() == 1 || ident.len() > 10 {
            return Kind::Identifier;
        }

        match ident {
            "if" => Kind::If,
            "while" => Kind::While,
            "for" => Kind::For,
            _ => Kind::Identifier,
        }
    }
}
