use crate::{
    eval::Eval,
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

/// Let statement ast node.
#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token, value: Box<Expression>) -> Self {
        assert_eq!(token.kind, TokenKind::Return, "expected return token");
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

impl Eval for ReturnStatement {
    fn eval(
        &self,
        _env: std::rc::Rc<std::cell::RefCell<crate::eval::ExecutionEnvironment>>,
    ) -> Result<std::rc::Rc<crate::eval::Value>, crate::eval::EvalError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Position;

    #[test]
    fn test_return_statement() {
        let token = Token::new(TokenKind::Return, Position(0, 0));
        let bool = Expression::new_boolean(Token::new(TokenKind::True, Position(0, 0)));

        let stmt = ReturnStatement::new(token.clone(), Box::new(bool));

        assert_eq!(stmt.token_literal(), token.literal());
        assert_eq!(stmt.to_string(), "return true");
    }
}
