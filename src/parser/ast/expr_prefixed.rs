use crate::lexer::Token;

use super::{Expression, Node};

/// Prefixed ast node.
#[derive(Debug)]
pub struct Prefixed {
    // Operator token
    pub op: Token,
    pub right: Box<dyn Expression>,
}

impl Prefixed {
    /// Creates a new prefixed node from a token.
    /// Asserts that the token is either `Token::Bang` or `Token::Minus`.
    pub fn new(op: Token, right: Box<dyn Expression>) -> Self {
        assert!(op == Token::Bang || op == Token::Minus);

        Self { op, right }
    }
}

impl ToString for Prefixed {
    fn to_string(&self) -> String {
        let mut out = self.op.literal();
        out.push_str(&self.right.to_string());
        out
    }
}

impl Node for Prefixed {
    fn token_literal(&self) -> String {
        self.op.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Prefixed {}
