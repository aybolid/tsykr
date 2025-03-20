use thiserror::Error;

use crate::lexer::{Lexer, Token};

use super::{Identifier, Node, Program};

#[derive(Debug, PartialEq, Error)]
pub enum ParserError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
        };
        parser.next_token();
        parser
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut program = Program::new();

        while self.current_token.is_some() {
            let stmt = self.parse_statement()?;
            program.push_statement(stmt);
            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Node>, ParserError> {
        if let Some(token) = self.current_token.take() {
            match token {
                Token::Identifier(_) => Ok(Box::new(Identifier::new(token))),
                _ => return Err(ParserError::UnexpectedToken(token)),
            }
        } else {
            return Err(ParserError::UnexpectedEndOfInput);
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next();
    }

    fn expect_peek_token<F>(&mut self, predicate: F) -> Result<(), ParserError>
    where
        F: Fn(&Token) -> bool,
    {
        match self.lexer.next() {
            Some(token) if predicate(&token) => Ok(()),
            Some(token) => Err(ParserError::UnexpectedToken(token)),
            None => Err(ParserError::UnexpectedEndOfInput),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_parser() {
        let lexer = Lexer::new("let".to_string());
        let parser = Parser::new(lexer);
        assert_eq!(parser.current_token, Some(Token::Let))
    }

    #[test]
    fn test_next_token() {
        let lexer = Lexer::new("let x = 5".to_string());
        let mut parser = Parser::new(lexer);
        parser.next_token();
        assert_eq!(
            parser.current_token,
            Some(Token::Identifier("x".to_string()))
        );
        parser.next_token();
        assert_eq!(parser.current_token, Some(Token::Equals));
        parser.next_token();
        assert_eq!(parser.current_token, Some(Token::Integer(5)));
    }

    #[test]
    fn test_parse_identifier() {
        let lexer = Lexer::new("name variable".to_string());
        let mut parser = Parser::new(lexer);
        let stmt = parser.parse_statement().unwrap();
        require_identifier(stmt, "name");
        parser.next_token();
        let stmt = parser.parse_statement().unwrap();
        require_identifier(stmt, "variable");
    }

    fn require_identifier(ident_node: Box<dyn Node>, expected: &str) {
        if let Some(ident) = ident_node.as_any().downcast_ref::<Identifier>() {
            assert_eq!(ident.token, Token::Identifier(expected.to_string()));
        } else {
            panic!("expected Identifier node");
        }
    }
}
