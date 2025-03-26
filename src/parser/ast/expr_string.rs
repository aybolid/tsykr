use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

use super::Node;

/// Float ast node.
#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub token: Token,
}

impl StringLiteral {
    /// Creates a new string node from a token.
    pub fn new(token: Token) -> Self {
        assert!(
            matches!(token.kind, TokenKind::String(_)),
            "expected string token"
        );

        Self { token }
    }
}

impl ToString for StringLiteral {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for StringLiteral {
    fn eval(&self, _env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        match &self.token.kind {
            TokenKind::String(value) => Ok(Value::new_string(value.to_string())),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        eval::{ExecutionEnvironment, Value},
        lexer::Position,
    };

    use super::*;

    #[test]
    fn test_string_node() {
        let token = Token::new(TokenKind::String("yeah".to_string()), Position(0, 0));
        let string = StringLiteral::new(token.clone());

        assert!(string.as_any().is::<StringLiteral>());
        assert_eq!(string.token, token);
        assert_eq!(string.to_string(), token.literal());
        assert_eq!(string.token_literal(), token.literal());
    }

    #[test]
    fn test_string_eval() {
        let token = Token::new(TokenKind::String("who".to_string()), Position(0, 0));
        let string = StringLiteral::new(token);

        let env = ExecutionEnvironment::new_global();
        let result = string.eval(env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::new_string("who".to_string()));
    }
}
