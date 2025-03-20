use crate::lexer::Token;

use super::{Node, Statement};

/// Let statement ast node.
#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    // TODO: add expression value
}

impl ReturnStatement {
    /// Creates a new return statement node.
    /// Asserts that the token is a `Token::Return`.
    pub fn new(token: Token) -> Self {
        assert_eq!(token, Token::Return, "expected return token");
        ReturnStatement { token }
    }
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        let out = String::from("return ");

        out
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for ReturnStatement {}
