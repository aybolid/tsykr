use crate::lexer::Token;

use super::{Expression, Node, Statement};

/// Let statement ast node.
#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<dyn Expression>,
}

impl ReturnStatement {
    /// Creates a new return statement node.
    /// Asserts that the token is a `Token::Return`.
    pub fn new(token: Token, value: Box<dyn Expression>) -> Self {
        assert_eq!(token, Token::Return, "expected return token");
        ReturnStatement { token, value }
    }
}

impl ToString for ReturnStatement {
    fn to_string(&self) -> String {
        let mut out = self.token.literal();
        out.push_str(" ");
        out.push_str(&self.value.to_string());
        out
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Statement for ReturnStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Boolean;

    #[test]
    fn test_return_statement() {
        let token = Token::Return;
        let bool = Boolean::new(Token::True);

        let stmt = ReturnStatement::new(token.clone(), Box::new(bool));

        assert_eq!(stmt.token_literal(), token.literal());
        assert_eq!(stmt.to_string(), "return true");
    }
}
