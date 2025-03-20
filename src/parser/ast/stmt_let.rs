use crate::lexer::Token;

use super::{Identifier, Node};

/// Let statement ast node.
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    // TODO: add expression value
}

impl LetStatement {
    /// Creates a let statement node.
    /// Asserts that the token is a `Token::Let`.
    pub fn new(token: Token, identifier: Identifier) -> Self {
        assert_eq!(token, Token::Let, "expected let token");
        LetStatement { token, identifier }
    }
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        let mut out = String::from("let ");

        out.push_str(&self.identifier.to_string());
        out.push_str(" = ");

        out
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        "let"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
