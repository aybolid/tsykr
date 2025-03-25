use crate::lexer::Token;

use super::{Expression, Node};

/// Infixed ast node.
#[derive(Debug, PartialEq)]
pub struct Infixed {
    pub left: Box<Expression>,
    /// Operator token
    pub op: Token,
    pub right: Box<Expression>,
}

impl Infixed {
    /// Creates a new infixed node from a token.
    pub fn new(op: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
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

// #[cfg(test)]
// mod tests {
//     use crate::{
//         lexer::{Position, TokenKind},
//         parser::Integer,
//     };

//     use super::*;

//     #[test]
//     fn test_infixed_node() {
//         let token = Token::new(TokenKind::Minus, Position(0, 0));
//         let left = Box::new(Integer::new(Token::new(
//             TokenKind::Integer(42),
//             Position(0, 0),
//         )));
//         let right = Box::new(Integer::new(Token::new(
//             TokenKind::Integer(42),
//             Position(0, 0),
//         )));
//         let infixed = Infixed::new(token.clone(), left, right);

//         assert_eq!(infixed.op, token);
//         assert_eq!(infixed.to_string(), "(42-42)");
//         assert_eq!(infixed.token_literal(), token.literal());
//     }
// }
