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
