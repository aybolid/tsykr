use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

/// Index ast node.
#[derive(Debug, PartialEq)]
pub struct Index {
    pub token: Token,
    pub of: Box<Expression>,
    pub index: Box<Expression>,
}

impl Index {
    pub fn new(token: Token, of: Box<Expression>, index: Box<Expression>) -> Self {
        assert_eq!(
            token.kind,
            TokenKind::LeftBracket,
            "expected left bracket token"
        );
        Self { token, of, index }
    }
}

impl ToString for Index {
    fn to_string(&self) -> String {
        let mut out = self.of.to_string();
        out.push('[');
        out.push_str(&self.index.to_string());
        out.push(']');
        out
    }
}

impl Node for Index {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Index {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let of = self.of.eval(Rc::clone(&env))?;
        let index = self.index.eval(env)?;

        match &*of {
            Value::Array(arr) => {
                if let Value::Integer(int) = *index {
                    if int as usize >= arr.len() {
                        Err(EvalError::IndexOutOfBounds(
                            int as usize,
                            arr.len(),
                            self.token.position,
                        ))
                    } else {
                        Ok(arr[int as usize].clone())
                    }
                } else {
                    Err(EvalError::InvalidIndexExpression(
                        of.to_string(),
                        index.to_string(),
                        self.token.position,
                    ))
                }
            }
            Value::String(s) => {
                if let Value::Integer(int) = *index {
                    if int as usize >= s.len() {
                        Err(EvalError::IndexOutOfBounds(
                            int as usize,
                            s.len(),
                            self.token.position,
                        ))
                    } else {
                        Ok(Value::new_string(
                            s.chars().nth(int as usize).unwrap().to_string(),
                        ))
                    }
                } else {
                    Err(EvalError::InvalidIndexExpression(
                        of.to_string(),
                        index.to_string(),
                        self.token.position,
                    ))
                }
            }
            _ => Err(EvalError::InvalidIndexExpression(
                of.to_string(),
                index.to_string(),
                self.token.position,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{eval::TRUE, lexer::Position};

    use super::*;

    #[test]
    fn test_index_node() {
        let token = Token::new(TokenKind::LeftBracket, Position(0, 0));
        let arr = Expression::new_array(
            token.clone(),
            vec![Box::new(Expression::new_boolean(Token::new(
                TokenKind::True,
                Position(0, 0),
            )))],
        );

        let idx = Index::new(
            token.clone(),
            Box::new(arr),
            Box::new(Expression::new_integer(Token::new(
                TokenKind::Integer(0),
                Position(0, 0),
            ))),
        );

        assert!(idx.as_any().is::<Index>());
        assert_eq!(idx.token, token);
        assert_eq!(idx.to_string(), "[true][0]");
        assert_eq!(idx.token_literal(), token.literal());
    }

    #[test]
    fn test_index_eval() {
        let token = Token::new(TokenKind::LeftBracket, Position(0, 0));
        let arr = Expression::new_array(
            token.clone(),
            vec![Box::new(Expression::new_boolean(Token::new(
                TokenKind::True,
                Position(0, 0),
            )))],
        );

        let idx = Index::new(
            token.clone(),
            Box::new(arr),
            Box::new(Expression::new_integer(Token::new(
                TokenKind::Integer(0),
                Position(0, 0),
            ))),
        );

        let env = ExecutionEnvironment::new_global();
        let result = idx.eval(env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TRUE.rc());
    }
}
