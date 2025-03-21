use crate::lexer::Token;

use super::{Expression, Identifier, Node, Statement};

/// Let statement ast node.
#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub value: Box<dyn Expression>,
}

impl LetStatement {
    /// Creates a let statement node.
    /// Asserts that the token is a `Token::Let`.
    pub fn new(token: Token, identifier: Identifier, value: Box<dyn Expression>) -> Self {
        assert_eq!(token, Token::Let, "expected let token");
        LetStatement {
            token,
            identifier,
            value,
        }
    }
}

impl ToString for LetStatement {
    fn to_string(&self) -> String {
        let mut out = self.token.literal();
        out.push_str(" ");

        out.push_str(&self.identifier.to_string());
        out.push_str(" = ");
        out.push_str(&self.value.to_string());

        out
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for LetStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Integer;

    #[test]
    fn test_let_statement() {
        let token = Token::Let;
        let ident = Identifier::new(Token::Identifier("who_cares".to_string()));
        let expr = Integer::new(Token::Integer(69));

        let stmt = LetStatement::new(token.clone(), ident, Box::new(expr));

        assert_eq!(stmt.token_literal(), token.literal());
        assert_eq!(stmt.to_string(), "let who_cares = 69");
    }
}
