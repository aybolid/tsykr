use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

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

impl Eval for Infixed {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let left_value = self.left.eval(Rc::clone(&env))?;
        let right_value = self.right.eval(env)?;

        match (&*left_value, &*right_value) {
            (Value::String(left), Value::String(right)) => eval_string_infix(left, &self.op, right),
            (Value::Integer(left), Value::Integer(right)) => {
                eval_int_infix(*left, &self.op, *right)
            }
            (Value::Boolean(left), Value::Boolean(right)) => {
                eval_bool_infix(*left, &self.op, *right)
            }
            (Value::Float(left), Value::Float(right)) => eval_float_infix(*left, &self.op, *right),
            (Value::Integer(left), Value::Float(right)) => {
                eval_float_infix(*left as f64, &self.op, *right)
            }
            (Value::Float(left), Value::Integer(right)) => {
                eval_float_infix(*left, &self.op, *right as f64)
            }
            _ => Err(EvalError::InvalidInfixOperation(
                left_value.to_string(),
                self.op.literal(),
                right_value.to_string(),
                self.op.position,
            )),
        }
    }
}

fn eval_string_infix(left: &str, op: &Token, right: &str) -> Result<Rc<Value>, EvalError> {
    match op.kind {
        TokenKind::Plus => Ok(Value::new_string(format!("{left}{right}"))),
        TokenKind::EqualsEquals => Ok(Value::from_native_bool(left == right)),
        TokenKind::BangEquals => Ok(Value::from_native_bool(left != right)),
        _ => Err(EvalError::InvalidInfixOperation(
            left.to_string(),
            op.literal(),
            right.to_string(),
            op.position,
        )),
    }
}

fn eval_float_infix(left: f64, op: &Token, right: f64) -> Result<Rc<Value>, EvalError> {
    if op.kind == TokenKind::Slash && right == 0.0 {
        return Err(EvalError::DivisionByZero(
            left.to_string(),
            right.to_string(),
            op.position,
        ));
    };

    match op.kind {
        TokenKind::Plus => Ok(Value::new_float(left + right)),
        TokenKind::Minus => Ok(Value::new_float(left - right)),
        TokenKind::Asterisk => Ok(Value::new_float(left * right)),
        TokenKind::Slash => Ok(Value::new_float(left / right)),

        TokenKind::EqualsEquals => Ok(Value::from_native_bool(left == right)),
        TokenKind::BangEquals => Ok(Value::from_native_bool(left != right)),
        TokenKind::LessThan => Ok(Value::from_native_bool(left < right)),
        TokenKind::GreaterThan => Ok(Value::from_native_bool(left > right)),
        TokenKind::LessThanEquals => Ok(Value::from_native_bool(left <= right)),
        TokenKind::GreaterThanEquals => Ok(Value::from_native_bool(left >= right)),

        _ => Err(EvalError::InvalidInfixOperation(
            left.to_string(),
            op.literal(),
            right.to_string(),
            op.position,
        )),
    }
}

fn eval_int_infix(left: i64, op: &Token, right: i64) -> Result<Rc<Value>, EvalError> {
    if op.kind == TokenKind::Slash && right == 0 {
        return Err(EvalError::DivisionByZero(
            left.to_string(),
            right.to_string(),
            op.position,
        ));
    };

    match op.kind {
        TokenKind::Plus => Ok(Value::new_integer(left + right)),
        TokenKind::Minus => Ok(Value::new_integer(left - right)),
        TokenKind::Asterisk => Ok(Value::new_integer(left * right)),
        TokenKind::Slash => Ok(Value::new_integer(left / right)),

        TokenKind::EqualsEquals => Ok(Value::from_native_bool(left == right)),
        TokenKind::BangEquals => Ok(Value::from_native_bool(left != right)),
        TokenKind::LessThan => Ok(Value::from_native_bool(left < right)),
        TokenKind::GreaterThan => Ok(Value::from_native_bool(left > right)),
        TokenKind::LessThanEquals => Ok(Value::from_native_bool(left <= right)),
        TokenKind::GreaterThanEquals => Ok(Value::from_native_bool(left >= right)),

        _ => Err(EvalError::InvalidInfixOperation(
            left.to_string(),
            op.literal(),
            right.to_string(),
            op.position,
        )),
    }
}

fn eval_bool_infix(left: bool, op: &Token, right: bool) -> Result<Rc<Value>, EvalError> {
    match op.kind {
        TokenKind::EqualsEquals => Ok(Value::from_native_bool(left == right)),
        TokenKind::BangEquals => Ok(Value::from_native_bool(left != right)),
        _ => Err(EvalError::InvalidInfixOperation(
            left.to_string(),
            op.literal(),
            right.to_string(),
            op.position,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        eval::FALSE,
        lexer::{Position, TokenKind},
    };

    use super::*;

    #[test]
    fn test_infixed_node() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let left = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let right = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let infixed = Infixed::new(token.clone(), left, right);

        assert_eq!(infixed.op, token);
        assert_eq!(infixed.to_string(), "(42-42)");
        assert_eq!(infixed.token_literal(), token.literal());
    }

    #[test]
    fn test_infix_eval() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let left = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let right = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let infixed = Infixed::new(token, left, right);

        let env = ExecutionEnvironment::new_global();
        let result = infixed.eval(Rc::clone(&env));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::new_integer(0));

        let token = Token::new(TokenKind::Plus, Position(0, 0));
        let left = Box::new(Expression::new_float(Token::new(
            TokenKind::Float(42.2),
            Position(0, 0),
        )));
        let right = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let infixed = Infixed::new(token, left, right);

        let result = infixed.eval(Rc::clone(&env));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::new_float(84.2));

        let token = Token::new(TokenKind::EqualsEquals, Position(0, 0));
        let left = Box::new(Expression::new_boolean(Token::new(
            TokenKind::True,
            Position(0, 0),
        )));
        let right = Box::new(Expression::new_boolean(Token::new(
            TokenKind::False,
            Position(0, 0),
        )));
        let infixed = Infixed::new(token, left, right);

        let result = infixed.eval(Rc::clone(&env));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FALSE.rc());
    }
}
