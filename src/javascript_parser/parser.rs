use crate::javascript_parser::ast::{Node, Program, Statement};
use crate::javascript_parser::kind::Kind;
use crate::javascript_parser::lexer::Lexer;
use crate::javascript_parser::token::Token;

pub struct Parser<'a> {
    /// Source code
    source: &'a str,
    lexer: Lexer<'a>,
    /// Current Token consumed from the lexer
    cur_token: Token,
    /// The end range of the previous token
    prev_token_end: usize,
}

// Parser functions
impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            cur_token: Token::default(),
            prev_token_end: 0,
        }
    }

    pub fn parse(&mut self) -> Program {
        Program {
            node: Node {
                start: 0,
                end: self.source.len(),
            },
            body: vec![]
        }
    }

    fn parse_debugger_statement(&mut self) -> Statement {
        let node = self.start_node();
        self.bump_any();
        Statement::DebuggerStatement
        //TODO: Need to set some sort of finish_node value here and assign it to the statement
    }
}

// Parser helper functions
impl<'a> Parser<'a> {
    fn start_node(&self) -> Node {
        let token = self.cur_token();
        Node::new(token.start, 0)
    }

    fn finish_node(&self, node: Node) -> Node {
        Node::new(node.start, self.prev_token_end)
    }

    fn cur_token(&self) -> &Token {
        &self.cur_token
    }

    fn cur_kind(&self) -> Kind {
        self.cur_token.kind
    }

    /// Checks if the current index has token `Kind`
    fn at(&self, kind: Kind) -> bool {
        self.cur_kind() == kind
    }

    /// Advance if we are at `Kind`
    fn bump(&mut self, kind: Kind) {
        if self.at(kind) {
            self.advance();
        }
    }

    /// Advance any token
    fn bump_any(&mut self) {
        self.advance();
    }

    /// Advance and return true if we are at `Kind`, return false otherwise
    fn eat(&mut self, kind: Kind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }
        false
    }

    /// Move to the next token
    fn advance(&mut self) {
        let token = self.lexer.read_next_token();
        self.prev_token_end = self.cur_token.end;
        self.cur_token = token;
    }
}