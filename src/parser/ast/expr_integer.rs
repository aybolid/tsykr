use crate::{
    eval::Eval,
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
    fn eval(
        &self,
        _env: std::rc::Rc<std::cell::RefCell<crate::eval::ExecutionEnvironment>>,
    ) -> Result<std::rc::Rc<crate::eval::Value>, crate::eval::EvalError> {
        todo!()
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
}
