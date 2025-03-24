use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecEnvironment, Object},
    lexer::{Token, TokenKind},
};

use super::Node;

/// Identifier ast node.
#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub token: Token,
}

impl Identifier {
    /// Creates a new identifier from a token.
    /// Asserts that the token is a `Token::Identifier`.
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
    fn eval(&self, env: Rc<RefCell<ExecEnvironment>>) -> Result<Option<Rc<Object>>, EvalError> {
        match env.borrow().get(&self.token.literal()) {
            Some(obj) => Ok(Some(obj)),
            None => Err(EvalError::UnknownIdentifier(
                self.token.literal(),
                self.token.position,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Position;

    use super::*;

    #[test]
    fn test_identifier_node() {
        let token = Token::new(TokenKind::Identifier("cool".to_string()), Position(0, 0));
        let ident = Identifier::new(token.clone());

        assert_eq!(ident.token, token);
        assert_eq!(ident.to_string(), token.literal());
        assert_eq!(ident.token_literal(), token.literal());
    }
}
