use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value, FALSE, TRUE},
    lexer::{Token, TokenKind},
};

use super::Node;

/// Boolean ast node.
#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub token: Token,
}

impl Boolean {
    pub fn new(token: Token) -> Self {
        assert!(
            token.kind == TokenKind::True || token.kind == TokenKind::False,
            "expected true or false token"
        );
        Self { token }
    }
}

impl ToString for Boolean {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Boolean {
    fn eval(&self, _env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        match self.token.kind {
            TokenKind::True => Ok(TRUE.rc()),
            TokenKind::False => Ok(FALSE.rc()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Position;

    use super::*;

    #[test]
    fn test_boolean_node() {
        let token = Token::new(TokenKind::True, Position(0, 0));
        let boolean = Boolean::new(token.clone());

        assert!(boolean.as_any().is::<Boolean>());
        assert_eq!(boolean.token, token);
        assert_eq!(boolean.to_string(), token.literal());
        assert_eq!(boolean.token_literal(), token.literal());
    }

    #[test]
    fn test_bool_eval() {
        let token = Token::new(TokenKind::True, Position(0, 0));
        let boolean = Boolean::new(token);

        let env = ExecutionEnvironment::new_global();
        let result = boolean.eval(env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TRUE.rc());
    }
}
