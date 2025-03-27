use std::{cell::RefCell, rc::Rc};

use crate::{
    eval::{Environment, Eval, EvalError, ExecutionEnvironment, Value},
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub token: Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

impl FunctionCall {
    pub fn new(token: Token, function: Box<Expression>, arguments: Vec<Box<Expression>>) -> Self {
        assert_eq!(
            token.kind,
            TokenKind::LeftParen,
            "expected left parenthesis token"
        );
        Self {
            token,
            function,
            arguments,
        }
    }
}

impl ToString for FunctionCall {
    fn to_string(&self) -> String {
        let mut out = String::from(self.function.to_string());
        out.push('(');
        out.push_str(
            &self
                .arguments
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        out.push(')');
        out
    }
}

impl Node for FunctionCall {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for FunctionCall {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError> {
        let func = self.function.eval(Rc::clone(&env))?;

        let mut args = vec![];
        for arg in &self.arguments {
            let arg_value = arg.eval(Rc::clone(&env))?;
            if arg_value.is_void() {
                return Err(EvalError::VoidValueAsArgument(self.token.position));
            }
            args.push(arg_value);
        }

        let callee = match &*func {
            Value::Function(f) => f,
            Value::Builtin(builtin) => return builtin(args),
            _ => return Err(EvalError::NotAFunction(self.token.position)),
        };

        if args.len() != callee.params.len() {
            return Err(EvalError::WrongNumberOfArguments(
                callee.params.len(),
                args.len(),
                self.token.position,
            ));
        }

        let local_env = ExecutionEnvironment::new_local(Rc::clone(&callee.captured_env));
        for (param, arg) in callee.params.iter().zip(args) {
            local_env.borrow_mut().set(param.clone(), arg);
        }

        let result = callee.body.eval(local_env)?;
        Ok(result.unwrap_returned())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Position,
        parser::{Block, FunctionExpression, Statement},
    };

    use super::*;

    #[test]
    fn test_function_call() {
        let call = FunctionCall::new(
            Token::new(TokenKind::LeftParen, Position(0, 0)),
            Box::new(Expression::new_identifier(Token::new(
                TokenKind::Identifier("my_func".to_string()),
                Position(0, 0),
            ))),
            vec![Box::new(Expression::new_integer(Token::new(
                TokenKind::Integer(42),
                Position(0, 0),
            )))],
        );

        assert!(call.as_any().is::<FunctionCall>());
        assert_eq!(call.function.to_string(), "my_func");
        assert_eq!(call.token_literal(), "(");
        assert_eq!(call.arguments.len(), 1);
        assert_eq!(call.arguments[0].to_string(), "42");
        assert_eq!(call.to_string(), "my_func(42)");
    }

    #[test]
    fn test_call_eval() {
        let block = Block::new(
            Token::new(TokenKind::LeftCurly, Position(0, 0)),
            vec![Box::new(Statement::new_return(
                Token::new(TokenKind::Return, Position(0, 0)),
                Box::new(Expression::new_integer(Token::new(
                    TokenKind::Integer(42),
                    Position(0, 0),
                ))),
            ))],
        );
        let params = vec![];
        let function = FunctionExpression::new(
            Token::new(TokenKind::Function, Position(0, 0)),
            params,
            block,
        );

        let call = FunctionCall::new(
            Token::new(TokenKind::LeftParen, Position(0, 0)),
            Box::new(Expression::Function(function)),
            vec![],
        );

        let env = ExecutionEnvironment::new_global();
        let result = call.eval(env);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::new_integer(42));
    }
}
