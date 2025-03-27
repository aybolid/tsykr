use std::{cell::RefCell, rc::Rc};

use crate::eval::{Eval, EvalError, ExecutionEnvironment, Value, VOID};

use super::{Expression, Identifier, Node};

/// Assign statement ast node.
#[derive(Debug, PartialEq)]
pub struct AssignStatement {
    pub identifier: Identifier,
    pub value: Box<Expression>,
}

impl AssignStatement {
    pub fn new(identifier: Identifier, value: Box<Expression>) -> Self {
        Self { identifier, value }
    }
}

impl ToString for AssignStatement {
    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(&self.identifier.to_string());
        out.push_str(" = ");
        out.push_str(&self.value.to_string());
        out
    }
}

impl Node for AssignStatement {
    fn token_literal(&self) -> String {
        self.identifier.token_literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for AssignStatement {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let to_store = self.value.eval(Rc::clone(&env))?;
        if to_store == VOID.rc() {
            return Err(EvalError::TriedToStoreVoid(self.identifier.token.position));
        }
        let name = self.identifier.token_literal();
        if !env.borrow_mut().assign(&name, to_store) {
            return Err(EvalError::NotDefined(name, self.identifier.token.position));
        }

        Ok(VOID.rc())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        eval::Environment,
        lexer::{Position, Token, TokenKind},
    };

    #[test]
    fn test_assign_statement() {
        let ident = Identifier::new(Token::new(
            TokenKind::Identifier("who_cares".to_string()),
            Position(0, 0),
        ));
        let int = Expression::new_integer(Token::new(TokenKind::Integer(69), Position(0, 0)));

        let stmt = AssignStatement::new(ident, Box::new(int));

        assert_eq!(stmt.token_literal(), "who_cares");
        assert_eq!(stmt.to_string(), "who_cares = 69");
    }

    #[test]
    fn test_assign_eval() {
        let ident = Identifier::new(Token::new(
            TokenKind::Identifier("who_cares".to_string()),
            Position(0, 0),
        ));
        let int = Expression::new_integer(Token::new(TokenKind::Integer(69), Position(0, 0)));

        let stmt = AssignStatement::new(ident, Box::new(int));

        let global_env = ExecutionEnvironment::new_global();
        global_env
            .borrow_mut()
            .set("who_cares".to_string(), Value::new_integer(1));

        let local_env = ExecutionEnvironment::new_local(Rc::clone(&global_env));
        local_env
            .borrow_mut()
            .set("who_cares".to_string(), Value::new_integer(2));

        let result = stmt.eval(Rc::clone(&local_env));

        let global_stored = global_env.borrow().get("who_cares");
        let local_stored = local_env.borrow().get("who_cares");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), VOID.rc());
        assert_eq!(global_stored, Some(Value::new_integer(1)));
        assert_eq!(local_stored, Some(Value::new_integer(69)));
    }
}
