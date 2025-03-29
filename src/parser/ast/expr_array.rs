use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

/// Array ast node.
#[derive(Debug, PartialEq)]
pub struct Array {
    pub token: Token,
    pub elements: Vec<Box<Expression>>,
}

impl Array {
    pub fn new(token: Token, elements: Vec<Box<Expression>>) -> Self {
        assert_eq!(
            token.kind,
            TokenKind::LeftBracket,
            "expected left bracket token"
        );
        Self { token, elements }
    }
}

impl ToString for Array {
    fn to_string(&self) -> String {
        let mut out = self.token.literal();
        out.push_str(
            &self
                .elements
                .iter()
                .map(|el| el.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        out.push(']');
        out
    }
}

impl Node for Array {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Array {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let mut els = vec![];
        for el in &self.elements {
            els.push(el.eval(Rc::clone(&env))?);
        }
        Ok(Value::new_array(els))
    }
}

#[cfg(test)]
mod tests {
    use crate::{eval::TRUE, lexer::Position};

    use super::*;

    #[test]
    fn test_array_node() {
        let token = Token::new(TokenKind::LeftBracket, Position(0, 0));
        let arr = Array::new(
            token.clone(),
            vec![Box::new(Expression::new_boolean(Token::new(
                TokenKind::True,
                Position(0, 0),
            )))],
        );

        assert!(arr.as_any().is::<Array>());
        assert_eq!(arr.token, token);
        assert_eq!(arr.to_string(), "[true]");
        assert_eq!(arr.token_literal(), token.literal());
        assert_eq!(arr.elements.len(), 1);
    }

    #[test]
    fn test_array_eval() {
        let token = Token::new(TokenKind::LeftBracket, Position(0, 0));
        let arr = Array::new(
            token.clone(),
            vec![Box::new(Expression::new_boolean(Token::new(
                TokenKind::True,
                Position(0, 0),
            )))],
        );

        let env = ExecutionEnvironment::new_global();
        let result = arr.eval(env);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::new_array(vec![TRUE.rc()]))
    }
}
