use std::sync::Arc;

use crate::{
    eval::{BooleanObject, Eval, EvalError, ExecEnvironment, IntegerObject, Object},
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

/// Infixed ast node.
#[derive(Debug)]
pub struct Infixed {
    pub left: Box<dyn Expression>,
    /// Operator token
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

impl Eval for Infixed {
    fn eval(&self, env: &mut ExecEnvironment) -> Result<Option<Arc<Object>>, EvalError> {
        let left_operand = (self.left.eval(env)?).expect("expression eval always returns Some");
        let right_operand = (self.right.eval(env)?).expect("expression eval always returns Some");

        match (&*left_operand, &*right_operand) {
            (Object::INTEGER(left), Object::INTEGER(right)) => match self.op.kind {
                TokenKind::Plus => Ok(Some(Arc::new(IntegerObject::new_object(left.0 + right.0)))),
                TokenKind::Minus => Ok(Some(Arc::new(IntegerObject::new_object(left.0 - right.0)))),
                TokenKind::Asterisk => {
                    Ok(Some(Arc::new(IntegerObject::new_object(left.0 * right.0))))
                }
                TokenKind::Slash => Ok(Some(Arc::new(IntegerObject::new_object(left.0 / right.0)))),
                TokenKind::EqualsEquals => Ok(Some(Arc::new(BooleanObject::object_from_bool(
                    left.0 == right.0,
                )))),
                TokenKind::BangEquals => Ok(Some(Arc::new(BooleanObject::object_from_bool(
                    left.0 != right.0,
                )))),
                TokenKind::LessThan => Ok(Some(Arc::new(BooleanObject::object_from_bool(
                    left.0 < right.0,
                )))),
                TokenKind::GreaterThan => Ok(Some(Arc::new(BooleanObject::object_from_bool(
                    left.0 > right.0,
                )))),
                TokenKind::LessThanEquals => Ok(Some(Arc::new(BooleanObject::object_from_bool(
                    left.0 <= right.0,
                )))),
                TokenKind::GreaterThanEquals => Ok(Some(Arc::new(
                    BooleanObject::object_from_bool(left.0 >= right.0),
                ))),
                _ => Err(EvalError::InvalidInfixOperation {
                    operator: self.op.literal(),
                    left: self.left.to_string(),
                    right: self.right.to_string(),
                    position: self.op.position,
                }),
            },
            (Object::BOOLEAN(left), Object::BOOLEAN(right)) => match self.op.kind {
                TokenKind::EqualsEquals => Ok(Some(Arc::new(BooleanObject::object_from_bool(
                    left.0 == right.0,
                )))),
                TokenKind::BangEquals => Ok(Some(Arc::new(BooleanObject::object_from_bool(
                    left.0 != right.0,
                )))),
                _ => Err(EvalError::InvalidInfixOperation {
                    operator: self.op.literal(),
                    left: self.left.to_string(),
                    right: self.right.to_string(),
                    position: self.op.position,
                }),
            },
            _ => Err(EvalError::InvalidInfixOperation {
                operator: self.op.literal(),
                left: self.left.to_string(),
                right: self.right.to_string(),
                position: self.op.position,
            }),
        }
    }
}

impl Expression for Infixed {}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{Position, TokenKind},
        parser::Integer,
    };

    use super::*;

    #[test]
    fn test_infixed_node() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let left = Box::new(Integer::new(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let right = Box::new(Integer::new(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let infixed = Infixed::new(token.clone(), left, right);

        assert_eq!(infixed.op, token);
        assert_eq!(infixed.to_string(), "(42-42)");
        assert_eq!(infixed.token_literal(), token.literal());
    }
}
