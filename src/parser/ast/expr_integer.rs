use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

use super::Node;

/// Integer ast node.
#[derive(Debug, PartialEq, Clone)]
pub struct Integer {
    pub token: Token,
}

impl Integer {
    pub fn new(token: Token) -> Self {
        assert!(
            matches!(token.kind, TokenKind::Integer(_)),
            "expected integer token"
        );

        Self { token }
    }
}

impl ToString for Integer {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for Integer {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Integer {
    fn eval(&self, _env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        match self.token.kind {
            TokenKind::Integer(value) => Ok(Value::new_integer(value)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Position;

    use super::*;

    #[test]
    fn test_integer_node() {
        let token = Token::new(TokenKind::Integer(42), Position(0, 0));
        let integer = Integer::new(token.clone());

        assert!(integer.as_any().is::<Integer>());
        assert_eq!(integer.token, token);
        assert_eq!(integer.to_string(), token.literal());
        assert_eq!(integer.token_literal(), token.literal());
    }

    #[test]
    fn test_integer_eval() {
        let token = Token::new(TokenKind::Integer(42), Position(0, 0));
        let integer = Integer::new(token.clone());

        let env = ExecutionEnvironment::new_global();
        let result = integer.eval(Rc::clone(&env));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::new_integer(42));
    }
}
