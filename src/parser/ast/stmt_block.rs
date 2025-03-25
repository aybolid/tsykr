use super::{Node, Statement};
use crate::lexer::{Token, TokenKind};

/// Block statement node.
#[derive(Debug, PartialEq)]
pub struct Block {
    pub token: Token,
    pub statements: Vec<Box<Statement>>,
}

impl Block {
    pub fn new(token: Token, statements: Vec<Box<Statement>>) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Position,
        parser::{Expression, Identifier},
    };

    use super::*;

    #[test]
    fn test_block_statement() {
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

        assert!(block.as_any().is::<Block>());
        assert_eq!(
            block.token_literal(),
            Token::new(TokenKind::LeftCurly, Position(0, 0)).literal()
        );
        assert_eq!(block.to_string(), "{\n  let x = 5\n  return x\n}");
    }
}
