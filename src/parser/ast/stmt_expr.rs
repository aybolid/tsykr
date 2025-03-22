use crate::{
    eval::{Eval, EvalError, ExecEnvironment, Object},
    lexer::Token,
};

use super::{Expression, Node, Statement};

/// Expression statement ast node.
#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    /// Creates a expression statement node.
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}

impl ToString for ExpressionStatement {
    fn to_string(&self) -> String {
        self.expression.to_string()
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for ExpressionStatement {
    fn eval(&self, env: &ExecEnvironment) -> Result<Box<dyn Object>, EvalError> {
        self.expression.eval(env)
    }
}

impl Statement for ExpressionStatement {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Position, Token, TokenKind};
    use crate::parser::{Infixed, Integer};

    #[test]
    fn test_expression_statement() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let left = Box::new(Integer::new(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let right = Box::new(Integer::new(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let infixed = Infixed::new(token.clone(), left, right);
        let expression_statement = ExpressionStatement::new(token.clone(), Box::new(infixed));

        assert_eq!(expression_statement.token, token);
        assert_eq!(expression_statement.expression.to_string(), "(42-42)");
        assert_eq!(expression_statement.token_literal(), token.literal());
    }
}
