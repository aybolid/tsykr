use crate::lexer::Token;

use super::{Expression, Node};

/// Infixed ast node.
#[derive(Debug)]
pub struct Infixed {
    // Operator token
    pub left: Box<dyn Expression>,
    pub op: Token,
    pub right: Box<dyn Expression>,
}

impl Infixed {
    /// Creates a new infixed node from a token.
    pub fn new(op: Token, left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self { op, left, right }
    }
}

impl ToString for Infixed {
    fn to_string(&self) -> String {
        let mut out = String::from("(");
        out.push_str(&self.left.to_string());
        out.push_str(&self.op.literal());
        out.push_str(&self.right.to_string());
        out.push_str(")");
        out
    }
}

impl Node for Infixed {
    fn token_literal(&self) -> String {
        self.op.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Expression for Infixed {}
