use thiserror::Error;

use crate::lexer::{Lexer, Token};

use super::{Identifier, LetStatement, Node, Program, ReturnStatement};

#[derive(Debug, PartialEq, Error)]
pub enum ParserError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
    #[error("Unexpected token: wanted {wanted:?} but found {found:?}")]
    IWantThisNotThat { wanted: Token, found: Token },
    #[error("Unexpected end of input but wanted {0:?}")]
    WhyNothingIfIWantThis(Token),
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
                Token::Let => Ok(Box::new(self.parse_let_statement(token)?)),
                Token::Return => Ok(Box::new(self.parse_return_statement(token)?)),
                _ => return Err(ParserError::UnexpectedToken(token)),
            }
        } else {
            unreachable!()
        }
    }

    /// Parses a return statement
    ///
    /// `trigger_token` - The token that triggered the parsing of the return statement. So must be Token::Return.
    fn parse_return_statement(
        &mut self,
        trigger_token: Token,
    ) -> Result<ReturnStatement, ParserError> {
        if trigger_token != Token::Return {
            return Err(ParserError::IWantThisNotThat {
                wanted: Token::Return,
                found: trigger_token,
            });
        }

        // Skip value for now
        // TODO: parse expression
        while self.current_token != Some(Token::SemiColon) {
            self.next_token();
        }

        Ok(ReturnStatement::new(trigger_token))
    }

    /// Parses a let statement
    ///
    /// `trigger_token` - The token that triggered the parsing of the let statement. So must be Token::Let.
    fn parse_let_statement(&mut self, trigger_token: Token) -> Result<LetStatement, ParserError> {
        if trigger_token != Token::Let {
            return Err(ParserError::IWantThisNotThat {
                wanted: Token::Let,
                found: trigger_token,
            });
        }

        self.expect_next_token_fn(|t| matches!(t, &Token::Identifier(_)))?;
        let identifier_token = self.current_token.take().expect("expected identifier");
        self.expect_next_token(Token::Equals)?;

        // Skip value for now
        // TODO: parse expression
        while self.current_token != Some(Token::SemiColon) {
            self.next_token();
        }

        Ok(LetStatement::new(
            trigger_token,
            Identifier::new(identifier_token),
        ))
    }

    /// Populates the current token with the next token from the lexer.
    /// Advances the lexer to the next token.
    fn next_token(&mut self) {
        self.current_token = self.lexer.next();
    }

    /// Sets `parser.current_token` to the next token if it matches the `expected_token`, otherwise returns an error.
    /// Advances the lexer to the next token.
    ///
    /// Generally, it's better to use this method instead of `expect_next_token_fn` as it produces a more descriptive error message.
    fn expect_next_token(&mut self, expected_token: Token) -> Result<(), ParserError> {
        match self.lexer.next() {
            Some(token) if token == expected_token => {
                self.current_token = Some(token);
                Ok(())
            }
            Some(token) => Err(ParserError::IWantThisNotThat {
                wanted: expected_token,
                found: token,
            }),
            None => Err(ParserError::WhyNothingIfIWantThis(expected_token)),
        }
    }

    /// Sets `parser.current_token` to the next token if it matches the `predicate`, otherwise returns an error.
    /// Advances the lexer to the next token.
    ///
    /// Use `expect_next_token` if equality check is all you need.
    fn expect_next_token_fn<F>(&mut self, predicate: F) -> Result<(), ParserError>
    where
        F: Fn(&Token) -> bool,
    {
        match self.lexer.next() {
            Some(token) if predicate(&token) => {
                self.current_token = Some(token);
                Ok(())
            }
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
        parser.next_token();
        assert_eq!(parser.current_token, None);
    }

    #[test]
    fn test_expect_next_token() {
        let lexer = Lexer::new("let x = 5".to_string());
        let mut parser = Parser::new(lexer);
        parser
            .expect_next_token_fn(|token| token == &Token::Identifier("x".to_string()))
            .unwrap();
        assert_eq!(
            parser.current_token,
            Some(Token::Identifier("x".to_string()))
        );
        parser.expect_next_token(Token::Equals).unwrap();
        assert_eq!(parser.current_token, Some(Token::Equals));
        parser
            .expect_next_token_fn(|token| token == &Token::Integer(5))
            .unwrap();
        assert_eq!(parser.current_token, Some(Token::Integer(5)));
        if let Ok(()) = parser.expect_next_token(Token::If) {
            panic!("expect_next_token should fail at this point");
        }
    }

    // #[test]
    // fn test_parse_identifier() {
    //     let lexer = Lexer::new("name variable".to_string());
    //     let mut parser = Parser::new(lexer);
    //     let stmt = parser.parse_statement().unwrap();
    //     assert_identifier(stmt, "name");
    //     parser.next_token();
    //     let stmt = parser.parse_statement().unwrap();
    //     assert_identifier(stmt, "variable");
    // }

    #[test]
    fn test_parse_let_statement() {
        let lexer = Lexer::new("let x = 5; let y = 10;".to_string());
        let mut parser = Parser::new(lexer);
        let stmt = parser.parse_statement().unwrap();
        assert_let_statement(stmt, "x");
        parser.next_token();
        let stmt = parser.parse_statement().unwrap();
        assert_let_statement(stmt, "y");
    }

    #[test]
    fn test_parse_return_statement() {
        let lexer = Lexer::new("return 5; return 10;".to_string());
        let mut parser = Parser::new(lexer);
        let stmt = parser.parse_statement().unwrap();
        assert_return_statement(stmt);
        parser.next_token();
        let stmt = parser.parse_statement().unwrap();
        assert_return_statement(stmt);
    }

    fn assert_identifier(node: Box<dyn Node>, expected: &str) {
        if let Some(ident) = node.as_any().downcast_ref::<Identifier>() {
            assert_eq!(ident.token, Token::Identifier(expected.to_string()));
        } else {
            panic!("expected Identifier node");
        }
    }

    fn assert_return_statement(node: Box<dyn Node>) {
        if let Some(return_stmt) = node.as_any().downcast_ref::<ReturnStatement>() {
            assert_eq!(return_stmt.token, Token::Return);
        } else {
            panic!("expected ReturnStatement node");
        }
    }

    fn assert_let_statement(node: Box<dyn Node>, expected_identifier: &str) {
        if let Some(let_stmt) = node.as_any().downcast_ref::<LetStatement>() {
            assert_eq!(let_stmt.token, Token::Let);
            assert_identifier(
                Box::new(Identifier::new(Token::Identifier(
                    let_stmt.identifier.token_literal().to_string(),
                ))),
                expected_identifier,
            );
        } else {
            panic!("expected LetStatement node");
        }
    }
}
