use crate::lexer::Token;

use super::{Expression, Node};

/// Integer ast node.
#[derive(Debug, PartialEq)]
pub struct Integer {
    pub token: Token,
}

impl Integer {
    /// Creates a new integer node from a token.
    /// Asserts that the token is a `Token::Integer`.
    pub fn new(token: Token) -> Self {
        assert!(matches!(token, Token::Integer(_)), "expected integer token");

        Self { token }
    }
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for Integer {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Integer {}
