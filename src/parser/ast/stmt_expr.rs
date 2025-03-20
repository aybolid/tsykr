use crate::lexer::Token;

use super::{Expression, Node, Statement};

/// Expression statement ast node.
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    /// Creates a express statement node.
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for ExpressionStatement {}
