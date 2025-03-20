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

    pub fn parse(&mut self) -> Result<Program, Vec<ParserError>> {
        let mut program = Program::new();

        let mut errors = vec![];

        while self.current_token.is_some() {
            let stmt = match self.parse_statement() {
                Ok(stmt) => stmt,
                Err(err) => {
                    errors.push(err);
                    self.next_token();
                    continue;
                }
            };
            program.push_statement(stmt);
            self.next_token();
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(program)
    }

    /// Parses a statement starting from the current token.
    /// Calling this function takes the current token.
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

    /// Populates the current token with the next token from the lexer.
    /// Advances the lexer to the next token.
    fn next_token(&mut self) {
        self.current_token = self.lexer.next();
    }

    /// Returns the next token if it matches the predicate, otherwise returns an error.
    /// Calling this function will advance the lexer to the next token.
    fn expect_peek_token<F>(&mut self, predicate: F) -> Result<Token, ParserError>
    where
        F: Fn(&Token) -> bool,
    {
        match self.lexer.next() {
            Some(token) if predicate(&token) => Ok(token),
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
        assert_identifier(stmt, "name");
        parser.next_token();
        let stmt = parser.parse_statement().unwrap();
        assert_identifier(stmt, "variable");
    }

    fn assert_identifier(node: Box<dyn Node>, expected: &str) {
        if let Some(ident) = node.as_any().downcast_ref::<Identifier>() {
            assert_eq!(ident.token, Token::Identifier(expected.to_string()));
        } else {
            panic!("expected Identifier node");
        }
    }
}
