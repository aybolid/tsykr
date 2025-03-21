use crate::lexer::Token;

use super::{Expression, Node};

/// Float ast node.
#[derive(Debug, PartialEq)]
pub struct Float {
    pub token: Token,
}

impl Float {
    /// Creates a new float node from a token.
    /// Asserts that the token is a `Token::Float`.
    pub fn new(token: Token) -> Self {
        assert!(matches!(token, Token::Float(_)), "expected float token");

        Self { token }
    }
}

impl ToString for Float {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for Float {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Float {}
