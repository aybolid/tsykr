use std::sync::Arc;

use super::{Node, Statement};
use crate::{
    eval::{Eval, EvalError, ExecEnvironment, Object},
    lexer::{Token, TokenKind},
};

/// Block statement node.
#[derive(Debug)]
pub struct Block {
    pub token: Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Block {
    /// Creates a new block statement.
    ///
    /// Asserts that the `token` is a `Token::LeftCurly`.
    pub fn new(token: Token, statements: Vec<Box<dyn Statement>>) -> Self {
        assert_eq!(token.kind, TokenKind::LeftCurly);
        Block { token, statements }
    }
}

impl ToString for Block {
    fn to_string(&self) -> String {
        let mut result = String::from("{\n");
        for statement in &self.statements {
            result.push_str("  ");
            result.push_str(&statement.to_string());
            result.push('\n');
        }
        result.push('}');
        result
    }
}

impl Node for Block {
    fn token_literal(&self) -> String {
        self.token.literal()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Eval for Block {
    fn eval(&self, _env: &mut ExecEnvironment) -> Result<Option<Arc<Object>>, EvalError> {
        todo!()
    }
}

impl Statement for Block {}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Position,
        parser::{Identifier, Integer, LetStatement, ReturnStatement},
    };

    use super::*;

    #[test]
    fn test_block_statement() {
        let block = Block::new(
            Token::new(TokenKind::LeftCurly, Position(0, 0)),
            vec![
                Box::new(LetStatement::new(
                    Token::new(TokenKind::Let, Position(0, 0)),
                    Identifier::new(Token::new(
                        TokenKind::Identifier("x".to_string()),
                        Position(0, 0),
                    )),
                    Box::new(Integer::new(Token::new(
                        TokenKind::Integer(5),
                        Position(0, 0),
                    ))),
                )),
                Box::new(ReturnStatement::new(
                    Token::new(TokenKind::Return, Position(0, 0)),
                    Box::new(Identifier::new(Token::new(
                        TokenKind::Identifier("x".to_string()),
                        Position(0, 0),
                    ))),
                )),
            ],
        );

        assert!(block.as_any().is::<Block>());
        assert_eq!(
            block.token_literal(),
            Token::new(TokenKind::LeftCurly, Position(0, 0)).literal()
        );
        assert_eq!(block.to_string(), "{\n  let x = 5\n  return x\n}");
    }
}
