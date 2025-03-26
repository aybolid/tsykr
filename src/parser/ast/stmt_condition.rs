use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value, TRUE, VOID},
    lexer::{Token, TokenKind},
};

use super::{Block, Expression, Node};

/// Condition statement ast node.
#[derive(Debug, PartialEq)]
pub struct ConditionStatement {
    pub token: Token,
    pub condition: Box<Expression>,
    pub if_true: Rc<Block>,
    pub if_false: Option<Rc<Block>>,
}

impl ConditionStatement {
    pub fn new(
        token: Token,
        condition: Box<Expression>,
        if_true: Block,
        if_false: Option<Block>,
    ) -> Self {
        assert_eq!(token.kind, TokenKind::If, "expected if token");

        ConditionStatement {
            token,
            condition,
            if_true: Rc::new(if_true),
            if_false: if_false.map(Rc::new),
        }
    }
}

impl ToString for ConditionStatement {
    fn to_string(&self) -> String {
        let mut out = self.token.literal();
        out.push_str(" (");
        out.push_str(&self.condition.to_string());
        out.push_str(") ");
        out.push_str(&self.if_true.to_string());

        if let Some(if_false) = &self.if_false {
            out.push_str(" else ");
            out.push_str(&if_false.to_string());
        }

        out
    }
}

impl Node for ConditionStatement {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for ConditionStatement {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let bool_cond = self.condition.eval(Rc::clone(&env))?;

        if !matches!(&*bool_cond, Value::Boolean(_)) {
            return Err(EvalError::NonBooleanCondition(
                bool_cond.to_string(),
                self.token.position,
            ));
        }

        if bool_cond == TRUE.rc() {
            let local_env = ExecutionEnvironment::new_local(env);
            let mut result = self.if_true.eval(local_env)?;

            if !result.is_returned() {
                result = VOID.rc();
            }

            return Ok(result);
        }

        if let Some(if_false) = &self.if_false {
            let local_env = ExecutionEnvironment::new_local(env);
            let mut result = if_false.eval(local_env)?;

            if !result.is_returned() {
                result = VOID.rc();
            }

            return Ok(result);
        }

        return Ok(VOID.rc());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Position;

    #[test]
    fn test_condition_statement() {
        let token = Token::new(TokenKind::If, Position(0, 0));
        let cond = Expression::new_boolean(Token::new(TokenKind::True, Position(0, 0)));

        let if_true = Block::new(Token::new(TokenKind::LeftCurly, Position(0, 0)), vec![]);

        let stmt = ConditionStatement::new(token.clone(), Box::new(cond), if_true, None);

        assert_eq!(stmt.token_literal(), token.literal());
        assert_eq!(stmt.to_string(), "if (true) {\n}");
    }

    #[test]
    fn test_condition_statement_eval() {} // TODO
}
