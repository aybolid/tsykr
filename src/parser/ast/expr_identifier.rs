use crate::lexer::Token;

use super::{Expression, Node};

/// Identifier ast node.
#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
}

impl Identifier {
    /// Creates a new identifier from a token.
    /// Asserts that the token is a `Token::Identifier`.
    pub fn new(token: Token) -> Self {
        assert!(
            matches!(token, Token::Identifier(_)),
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

impl Expression for Identifier {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_node() {
        let token = Token::Identifier("cool".to_string());
        let ident = Identifier::new(token.clone());

        assert_eq!(ident.token, token);
        assert_eq!(ident.to_string(), token.literal());
        assert_eq!(ident.token_literal(), token.literal());
    }
}
