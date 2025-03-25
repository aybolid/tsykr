use crate::lexer::{Token, TokenKind};

use super::{Expression, Node};

/// Prefixed ast node.
#[derive(Debug, PartialEq)]
pub struct Prefixed {
    /// Operator token
    pub op: Token,
    pub right: Box<Expression>,
}

impl Prefixed {
    pub fn new(op: Token, right: Box<Expression>) -> Self {
        assert!(
            op.kind == TokenKind::Bang || op.kind == TokenKind::Minus,
            "expected bang or minus token"
        );

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

// #[cfg(test)]
// mod tests {
//     use crate::{lexer::Position, parser::Integer};

//     use super::*;

//     #[test]
//     fn test_prefixed_node() {
//         let token = Token::new(TokenKind::Minus, Position(0, 0));
//         let right = Box::new(Integer::new(Token::new(
//             TokenKind::Integer(42),
//             Position(0, 0),
//         )));
//         let prefixed = Prefixed::new(token.clone(), right);

//         assert_eq!(prefixed.op, token);
//         assert_eq!(prefixed.to_string(), "(-42)");
//         assert_eq!(prefixed.token_literal(), token.literal());
//     }
// }
