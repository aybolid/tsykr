use std::sync::Arc;

use crate::{
    eval::{Eval, EvalError, ExecEnvironment, IntegerObject, Object, FALSE, TRUE},
    lexer::{Token, TokenKind},
};

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
    fn eval(&self, env: &mut ExecEnvironment) -> Result<Option<Arc<Object>>, EvalError> {
        let operand = (self.right.eval(env)?).expect("all expressions return smth");
        match self.op.kind {
            TokenKind::Bang => match &*operand {
                Object::BOOLEAN(b) => match b.0 {
                    true => Ok(Some(Arc::new(FALSE))),
                    false => Ok(Some(Arc::new(TRUE))),
                },
                _ => Err(EvalError::InvalidPrefixOperation {
                    operator: self.op.literal(),
                    operand: operand.inspect(),
                    position: self.op.position,
                }),
            },
            TokenKind::Minus => match &*operand {
                Object::INTEGER(i) => Ok(Some(Arc::new(Object::INTEGER(IntegerObject(-i.0))))),
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

impl Expression for Prefixed {}

#[cfg(test)]
mod tests {
    use crate::{lexer::Position, parser::Integer};

    use super::*;

    #[test]
    fn test_prefixed_node() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let right = Box::new(Integer::new(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let prefixed = Prefixed::new(token.clone(), right);

        assert_eq!(prefixed.op, token);
        assert_eq!(prefixed.to_string(), "(-42)");
        assert_eq!(prefixed.token_literal(), token.literal());
    }
}
