use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecEnvironment, Object},
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

/// Prefixed ast node.
#[derive(Debug, PartialEq, Clone)]
pub struct Prefixed {
    /// Operator token
    pub op: Token,
    pub right: Box<Expression>,
}

impl Prefixed {
    /// Creates a new prefixed node from a token.
    /// Asserts that the token is either `Token::Bang` or `Token::Minus`.
    pub fn new(op: Token, right: Box<Expression>) -> Self {
        assert!(op.kind == TokenKind::Bang || op.kind == TokenKind::Minus);

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

impl Eval for Prefixed {
    fn eval(&self, env: Rc<RefCell<ExecEnvironment>>) -> Result<Option<Rc<Object>>, EvalError> {
        let operand = (self.right.eval(env)?).expect("all expressions return smth");
        match self.op.kind {
            TokenKind::Bang => match &*operand {
                Object::BOOLEAN(b) => Ok(Some(Rc::new(b.negated_object()))),
                _ => Err(EvalError::InvalidPrefixOperation {
                    operator: self.op.literal(),
                    operand: operand.inspect(),
                    position: self.op.position,
                }),
            },
            TokenKind::Minus => match &*operand {
                Object::INTEGER(i) => Ok(Some(Rc::new(i.negated_object()))),
                Object::FLOAT(f) => Ok(Some(Rc::new(f.negated_object()))),
                _ => Err(EvalError::InvalidPrefixOperation {
                    operator: self.op.literal(),
                    operand: operand.inspect(),
                    position: self.op.position,
                }),
            },
            _ => unreachable!(),
        }
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
