use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::Token,
};

use super::{Expression, Node};

/// Expression statement ast node.
#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<Expression>,
}

impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<Expression>) -> Self {
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
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        self.expression.eval(env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Position, Token, TokenKind};

    #[test]
    fn test_expression_statement() {
        let token = Token::new(TokenKind::Minus, Position(0, 0));
        let left = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let right = Box::new(Expression::new_integer(Token::new(
            TokenKind::Integer(42),
            Position(0, 0),
        )));
        let infixed = Expression::new_infixed(token.clone(), left, right);
        let expression_statement = ExpressionStatement::new(token.clone(), Box::new(infixed));

        assert_eq!(expression_statement.token, token);
        assert_eq!(expression_statement.expression.to_string(), "(42-42)");
        assert_eq!(expression_statement.token_literal(), token.literal());
    }
}
