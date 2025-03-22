use super::{Node, Statement};
use crate::lexer::Token;

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
        assert_eq!(token, Token::LeftCurly);
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

impl Statement for Block {}

#[cfg(test)]
mod tests {
    use crate::parser::{Identifier, Integer, LetStatement, ReturnStatement};

    use super::*;

    #[test]
    fn test_block_statement() {
        let block = Block::new(
            Token::LeftCurly,
            vec![
                Box::new(LetStatement::new(
                    Token::Let,
                    Identifier::new(Token::Identifier("x".to_string())),
                    Box::new(Integer::new(Token::Integer(5))),
                )),
                Box::new(ReturnStatement::new(
                    Token::Return,
                    Box::new(Identifier::new(Token::Identifier("x".to_string()))),
                )),
            ],
        );

        assert!(block.as_any().is::<Block>());
        assert_eq!(block.token_literal(), Token::LeftCurly.literal());
        assert_eq!(block.to_string(), "{\n  let x = 5\n  return x\n}");
    }
}
