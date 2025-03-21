use crate::lexer::Token;

use super::{Expression, Node};

/// Boolean ast node.
#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
}

impl Boolean {
    /// Creates a new boolean node from a token.
    /// Asserts that the token is either `Token::True` or `Token::False`.
    pub fn new(token: Token) -> Self {
        assert!(token == Token::True || token == Token::False);

        Self { token }
    }
}

impl ToString for Boolean {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Boolean {}
