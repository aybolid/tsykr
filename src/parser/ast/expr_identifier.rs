use crate::lexer::Token;

use super::Node;

/// Identifier ast node.
#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
}

impl Identifier {
    /// Creates a new identifier from a token.
    /// Asserts that the token is a `Token::Identifier`.
    pub fn new(token: Token) -> Self {
        assert!(
            matches!(token, Token::Identifier(_)),
            "expected identifier token"
        );

        Identifier { token }
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        match &self.token {
            Token::Identifier(name) => name.clone(),
            _ => unreachable!(),
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        match &self.token {
            Token::Identifier(name) => name,
            _ => unreachable!(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
