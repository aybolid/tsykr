use crate::lexer::Token;

use super::{Expression, Node};

/// Integer ast node.
#[derive(Debug)]
pub struct Integer {
    pub token: Token,
}

impl Integer {
    /// Creates a new integer node from a token.
    /// Asserts that the token is a `Token::Integer`.
    pub fn new(token: Token) -> Self {
        assert!(matches!(token, Token::Integer(_)), "expected integer token");

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

impl Expression for Integer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_node() {
        let token = Token::Integer(42);
        let integer = Integer::new(token.clone());

        assert!(integer.as_any().is::<Integer>());
        assert_eq!(integer.token, token);
        assert_eq!(integer.to_string(), token.literal());
        assert_eq!(integer.token_literal(), token.literal());
    }
}
