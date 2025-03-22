use crate::{
    eval::Eval,
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

/// Float ast node.
#[derive(Debug)]
pub struct Float {
    pub token: Token,
}

impl Float {
    /// Creates a new float node from a token.
    /// Asserts that the token is a `Token::Float`.
    pub fn new(token: Token) -> Self {
        assert!(
            matches!(token.kind, TokenKind::Float(_)),
            "expected float token"
        );

        Self { token }
    }
}

impl ToString for Float {
    fn to_string(&self) -> String {
        self.token.literal()
    }
}

impl Node for Float {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Float {
    fn eval(&self) -> Result<Box<dyn crate::eval::Object>, crate::eval::EvalError> {
        todo!()
    }
}

impl Expression for Float {}

#[cfg(test)]
mod tests {
    use crate::lexer::Position;

    use super::*;

    #[test]
    fn test_float_node() {
        let token = Token::new(TokenKind::Float(2.23), Position(0, 0));
        let float = Float::new(token.clone());

        assert!(float.as_any().is::<Float>());
        assert_eq!(float.token, token);
        assert_eq!(float.to_string(), token.literal());
        assert_eq!(float.token_literal(), token.literal());
    }
}
