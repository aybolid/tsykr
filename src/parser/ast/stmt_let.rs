use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Environment, Eval, EvalError, ExecutionEnvironment, Value, VOID},
    lexer::{Token, TokenKind},
};

use super::{Expression, Identifier, Node};

/// Let statement ast node.
#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub identifier: Identifier,
    pub value: Box<Expression>,
}

impl LetStatement {
    pub fn new(token: Token, identifier: Identifier, value: Box<Expression>) -> Self {
        assert_eq!(token.kind, TokenKind::Let, "expected let token");
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

impl Eval for LetStatement {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let to_store = self.value.eval(Rc::clone(&env))?;

        if to_store == VOID.rc() {
            return Err(EvalError::TriedToStoreVoid(self.token.position));
        }

        let name = self.identifier.token_literal();

        env.borrow_mut().set(name, to_store);

        Ok(VOID.rc())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{eval::Environment, lexer::Position};

    #[test]
    fn test_let_statement() {
        let token = Token::new(TokenKind::Let, Position(0, 0));
        let ident = Identifier::new(Token::new(
            TokenKind::Identifier("who_cares".to_string()),
            Position(0, 0),
        ));
        let int = Expression::new_integer(Token::new(TokenKind::Integer(69), Position(0, 0)));

        let stmt = LetStatement::new(token.clone(), ident, Box::new(int));

        assert_eq!(stmt.token_literal(), token.literal());
        assert_eq!(stmt.to_string(), "let who_cares = 69");
    }

    #[test]
    fn test_let_eval() {
        let token = Token::new(TokenKind::Let, Position(0, 0));
        let ident = Identifier::new(Token::new(
            TokenKind::Identifier("who_cares".to_string()),
            Position(0, 0),
        ));
        let int = Expression::new_integer(Token::new(TokenKind::Integer(69), Position(0, 0)));

        let stmt = LetStatement::new(token, ident, Box::new(int));

        let env = ExecutionEnvironment::new_global();
        let result = stmt.eval(Rc::clone(&env));
        let stored = env.borrow().get("who_cares");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), VOID.rc());
        assert_eq!(stored, Some(Value::new_integer(69)))
    }
}
