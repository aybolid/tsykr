use crate::{
    eval::Eval,
    lexer::{Token, TokenKind},
};

use super::{Expression, Node};

#[derive(Debug)]
pub struct FunctionCall {
    pub token: Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl FunctionCall {
    pub fn new(
        token: Token,
        function: Box<dyn Expression>,
        arguments: Vec<Box<dyn Expression>>,
    ) -> Self {
        assert_eq!(token.kind, TokenKind::LeftParen);
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
    fn eval(&self) -> Result<Box<dyn crate::eval::Object>, crate::eval::EvalError> {
        todo!()
    }
}

impl Expression for FunctionCall {}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Position,
        parser::{Identifier, Integer},
    };

    use super::*;

    #[test]
    fn test_function_call() {
        let call = FunctionCall::new(
            Token::new(TokenKind::LeftParen, Position(0, 0)),
            Box::new(Identifier::new(Token::new(
                TokenKind::Identifier("my_func".to_string()),
                Position(0, 0),
            ))),
            vec![Box::new(Integer::new(Token::new(
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
}
