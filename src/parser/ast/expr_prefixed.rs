use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

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

impl Eval for Prefixed {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let value = self.right.eval(env)?;

        match self.op.kind {
            TokenKind::Bang => match &*value {
                Value::Boolean(b) => Ok(Value::from_native_bool(!b)),
                _ => Err(EvalError::InvalidPrefixOperation(
                    self.op.literal(),
                    value.to_string(),
                    self.op.position,
                )),
            },
            TokenKind::Minus => match &*value {
                Value::Integer(i) => Ok(Value::new_integer(-i)),
                Value::Float(f) => Ok(Value::new_float(-f)),
                _ => Err(EvalError::InvalidPrefixOperation(
                    self.op.literal(),
                    value.to_string(),
                    self.op.position,
                )),
            },
            _ => Err(EvalError::InvalidPrefixOperation(
                self.op.literal(),
                value.to_string(),
                self.op.position,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{eval::FALSE, lexer::Position, parser::Integer};

    use super::*;

    #[test]
    fn test_prefixed_node() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let right = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let prefixed = Prefixed::new(token.clone(), right);

        assert_eq!(prefixed.op, token);
        assert_eq!(prefixed.to_string(), "(-42)");
        assert_eq!(prefixed.token_literal(), token.literal());
    }

    #[test]
    fn test_prefixed_eval() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let right = Box::new(Expression::Integer(Integer::new(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        ))));
        let prefixed = Prefixed::new(token, right);

        let env = ExecutionEnvironment::new_global();
        let result = prefixed.eval(Rc::clone(&env));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::new_integer(-42));

        let token = Token::new(TokenKind::Bang, Position(0, 0));
        let right = Box::new(Expression::new_boolean(Token::new(
            TokenKind::True,
            Position(0, 0),
        )));
        let prefixed = Prefixed::new(token, right);

        let result = prefixed.eval(env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FALSE.rc());
    }
}
