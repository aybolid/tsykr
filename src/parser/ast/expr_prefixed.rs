use crate::lexer::Token;

use super::{Expression, Node};

/// Prefixed ast node.
#[derive(Debug)]
pub struct Prefixed {
    /// Operator token
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
        let mut out = String::from("(");
        out.push_str(&self.op.literal());
        out.push_str(&self.right.to_string());
        out.push_str(")");
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

#[cfg(test)]
mod tests {
    use crate::parser::Integer;

    use super::*;

    #[test]
    fn test_prefixed_node() {
        let token = Token::Minus;
        let right = Box::new(Integer::new(Token::Integer(42)));
        let prefixed = Prefixed::new(token.clone(), right);

        assert_eq!(prefixed.op, token);
        assert_eq!(prefixed.to_string(), "(-42)");
        assert_eq!(prefixed.token_literal(), token.literal());
    }
}
