use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Environment, Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

use super::Node;

/// Identifier ast node.
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        assert!(
            matches!(token.kind, TokenKind::Identifier(_)),
            "expected identifier token"
        );

        Identifier { token }
    }
}

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Identifier {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let name = self.token.literal();
        match env.borrow().get(&name) {
            Some(value) => Ok(value),
            None => Err(EvalError::NotDefined(name, self.token.position)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{eval::TRUE, lexer::Position};

    use super::*;

    #[test]
    fn test_identifier_node() {
        let token = Token::new(TokenKind::Identifier("cool".to_string()), Position(0, 0));
        let ident = Identifier::new(token.clone());

        assert_eq!(ident.token, token);
        assert_eq!(ident.to_string(), token.literal());
        assert_eq!(ident.token_literal(), token.literal());
    }

    #[test]
    fn test_eval_identifier() {
        let token = Token::new(TokenKind::Identifier("cool".to_string()), Position(0, 0));
        let ident = Identifier::new(token.clone());

        let env = ExecutionEnvironment::new_global();
        env.borrow_mut().set(token.literal(), TRUE.rc());

        let result = ident.eval(env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TRUE.rc());
    }
}
