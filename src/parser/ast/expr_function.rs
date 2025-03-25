use std::{cell::RefCell, rc::Rc};

use super::{Block, Identifier, Node};
use crate::{
    eval::{Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

#[derive(Debug, PartialEq)]
pub struct FunctionExpression {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Rc<Block>,
}

impl FunctionExpression {
    pub fn new(token: Token, parameters: Vec<Identifier>, body: Block) -> Self {
        assert_eq!(token.kind, TokenKind::Function, "expected function token");
        FunctionExpression {
            token,
            parameters,
            body: Rc::new(body),
        }
    }
}

impl ToString for FunctionExpression {
    fn to_string(&self) -> String {
        let mut out = String::from(self.token_literal());
        out.push(' ');

        out.push('(');
        out.push_str(
            &self
                .parameters
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        out.push_str(") ");
        out.push_str(&self.body.to_string());

        out
    }
}

impl Node for FunctionExpression {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for FunctionExpression {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let params = self
            .parameters
            .iter()
            .map(|ident| ident.to_string())
            .collect::<Vec<String>>();
        Ok(Value::new_function(params, Rc::clone(&self.body), env))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Position,
        parser::{Expression, Statement},
    };

    use super::*;

    #[test]
    fn test_function_expression() {
        let block = Block::new(
            Token::new(TokenKind::LeftCurly, Position(0, 0)),
            vec![
                Box::new(Statement::new_let(
                    Token::new(TokenKind::Let, Position(0, 0)),
                    Identifier::new(Token::new(
                        TokenKind::Identifier("x".to_string()),
                        Position(0, 0),
                    )),
                    Box::new(Expression::new_integer(Token::new(
                        TokenKind::Integer(5),
                        Position(0, 0),
                    ))),
                )),
                Box::new(Statement::new_return(
                    Token::new(TokenKind::Return, Position(0, 0)),
                    Box::new(Expression::new_identifier(Token::new(
                        TokenKind::Identifier("x".to_string()),
                        Position(0, 0),
                    ))),
                )),
            ],
        );
        let params = vec![
            Identifier::new(Token::new(
                TokenKind::Identifier("_a".to_string()),
                Position(0, 0),
            )),
            Identifier::new(Token::new(
                TokenKind::Identifier("_b".to_string()),
                Position(0, 0),
            )),
        ];
        let function = FunctionExpression::new(
            Token::new(TokenKind::Function, Position(0, 0)),
            params,
            block,
        );

        assert!(function.as_any().is::<FunctionExpression>());
        assert_eq!(
            function.token,
            Token::new(TokenKind::Function, Position(0, 0))
        );
        assert_eq!(function.token_literal(), function.token.literal());
        assert_eq!(
            function.to_string(),
            "fn (_a, _b) {\n  let x = 5\n  return x\n}"
        )
    }

    #[test]
    fn test_fn_expr_eval() {
        let block = Block::new(Token::new(TokenKind::LeftCurly, Position(0, 0)), vec![]);
        let params = vec![];
        let function = FunctionExpression::new(
            Token::new(TokenKind::Function, Position(0, 0)),
            params,
            block,
        );

        let env = ExecutionEnvironment::new_global();
        let result = function.eval(env);

        assert!(result.is_ok());
        let matched = matches!(&*result.unwrap(), Value::Function(_));
        assert_eq!(matched, true);
    }
}
