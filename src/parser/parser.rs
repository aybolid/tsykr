use thiserror::Error;

use crate::lexer::{Lexer, Token};

use super::{
    precedence::Precedence, Expression, ExpressionStatement, Identifier, LetStatement, Program,
    ReturnStatement, Statement,
};

#[derive(Debug, PartialEq, Error)]
pub enum ParserError {
    #[error("Unexpected token: {0:?}")]
    IDontWantThis(Token),
    #[error("Unexpected token: wanted {expected:?} but found {actual:?}")]
    IWantThisNotThat {
        expected: Token,
        actual: Option<Token>,
    },
    #[error("Unexpected end of input but wanted {0:?}")]
    WhyNothingIfIWantThis(Token),
    #[error("Unexpected end of input")]
    WhereIsEverybody,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };
        parser.next_token();
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
    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParserError> {
        if let Some(ref token) = self.current_token {
            println!("{token:?}");
            match token {
                Token::Let => Ok(Box::new(self.parse_let_statement()?)),
                Token::Return => Ok(Box::new(self.parse_return_statement()?)),
                Token::ILLEGAL(_) => Err(ParserError::IDontWantThis(token.clone())),
                _ => Ok(Box::new(self.parse_expression_statement()?)),
            }
        } else {
            unreachable!()
        }
    }

    /// Parses an expression statement
    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, ParserError> {
        if self.current_token.is_none() {
            return Err(ParserError::WhereIsEverybody);
        }

        Ok(ExpressionStatement::new(
            // clone token as it will be consumed by the expression parser
            self.current_token.clone().expect("checked before"),
            self.parse_expression(Precedence::Lowest)?,
        ))
    }

    fn parse_expression(
        &mut self,
        _precedence: Precedence,
    ) -> Result<Box<dyn Expression>, ParserError> {
        if self.current_token.is_none() {
            return Err(ParserError::WhereIsEverybody);
        }

        let left_expr = match self.current_token.as_ref().expect("checked before") {
            Token::Identifier(_) => self.parse_identifier()?,
            _ => todo!(),
        };

        Ok(Box::new(left_expr))
    }

    /// Parses an identifier.
    ///
    /// Takes the current token and returns an Identifier.
    fn parse_identifier(&mut self) -> Result<Identifier, ParserError> {
        self.expect_current_token_fn(|t| matches!(t, &Token::Identifier(_)))?;

        let token = self.current_token.take().expect("checked before");
        let identifier = Identifier::new(token);

        Ok(identifier)
    }

    /// Parses a return statement
    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParserError> {
        self.expect_current_token(Token::Return)?;
        let return_token = self.current_token.take().expect("checked before");
        self.next_token();

        // Skip value for now
        // TODO: parse expression
        while !matches!(self.current_token, Some(Token::SemiColon)) {
            self.next_token();
        }

        Ok(ReturnStatement::new(return_token))
    }

    /// Parses a let statement
    fn parse_let_statement(&mut self) -> Result<LetStatement, ParserError> {
        self.expect_current_token(Token::Let)?;
        let let_token = self.current_token.take().expect("checked before");
        self.next_token();

        let identifier = self.parse_identifier()?;

        self.expect_peek_token(Token::Equals)?;
        self.next_token();

        // Skip value for now
        // TODO: parse expression
        while !matches!(self.current_token, Some(Token::SemiColon)) {
            self.next_token();
        }

        Ok(LetStatement::new(let_token, identifier))
    }

    /// Populates the current token and the peek token.
    /// Advances the lexer to the next token.
    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
    }

    /// Checks if the peek token matches the expected token.
    ///
    /// Generally, it's better to use this method instead of `expect_next_token_fn` as it produces a more descriptive error message.
    fn expect_peek_token(&mut self, expected_token: Token) -> Result<(), ParserError> {
        match &self.peek_token {
            Some(token) if token == &expected_token => Ok(()),
            Some(token) => Err(ParserError::IWantThisNotThat {
                expected: expected_token,
                actual: Some(token.clone()),
            }),
            None => Err(ParserError::WhyNothingIfIWantThis(expected_token)),
        }
    }

    fn expect_current_token(&mut self, expected_token: Token) -> Result<(), ParserError> {
        match &self.current_token {
            Some(token) if token == &expected_token => Ok(()),
            Some(token) => Err(ParserError::IWantThisNotThat {
                expected: expected_token,
                actual: Some(token.clone()),
            }),
            None => Err(ParserError::WhyNothingIfIWantThis(expected_token)),
        }
    }

    fn expect_current_token_fn<F>(&mut self, predicate: F) -> Result<(), ParserError>
    where
        F: Fn(&Token) -> bool,
    {
        match &self.current_token {
            Some(token) if predicate(token) => Ok(()),
            Some(token) => Err(ParserError::IDontWantThis(token.clone())),
            None => Err(ParserError::WhereIsEverybody),
        }
    }

    /// Checks if the peek token matches the predicate.
    ///
    /// Use `expect_next_token` if equality check is all you need.
    #[allow(unused)]
    fn expect_peek_token_fn<F>(&mut self, predicate: F) -> Result<(), ParserError>
    where
        F: Fn(&Token) -> bool,
    {
        match &self.peek_token {
            Some(token) if predicate(token) => Ok(()),
            Some(token) => Err(ParserError::IDontWantThis(token.clone())),
            None => Err(ParserError::WhereIsEverybody),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Expression, Node};

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
    fn test_expect_peek_token() {
        let lexer = Lexer::new("let x = 5".to_string());
        let mut parser = Parser::new(lexer);

        println!("{parser:?}");

        parser
            .expect_peek_token_fn(|token| token == &Token::Identifier("x".to_string()))
            .unwrap();
        assert_eq!(parser.peek_token, Some(Token::Identifier("x".to_string())));
        parser.next_token();

        parser.expect_peek_token(Token::Equals).unwrap();
        assert_eq!(parser.peek_token, Some(Token::Equals));
        parser.next_token();

        parser
            .expect_peek_token_fn(|token| token == &Token::Integer(5))
            .unwrap();
        assert_eq!(parser.peek_token, Some(Token::Integer(5)));
        if let Ok(()) = parser.expect_peek_token(Token::If) {
            panic!("expect_next_token should fail at this point");
        }
    }

    #[test]
    fn test_parse_identifier() {
        let lexer = Lexer::new("name variable".to_string());
        let mut parser = Parser::new(lexer);

        let stmt = parser.parse_statement().unwrap();
        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
            assert_eq!(expr_stmt.expression.token_literal(), "name");
        } else {
            panic!("expected ExpressionStatement node");
        }

        parser.next_token();
        let stmt = parser.parse_statement().unwrap();
        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
            assert_eq!(expr_stmt.expression.token_literal(), "variable");
        } else {
            panic!("expected ExpressionStatement node");
        }
    }

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

    #[test]
    fn test_parser_errors() {
        let lexer = Lexer::new("№".to_string());
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Err(errors) => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0], ParserError::IDontWantThis(Token::ILLEGAL('№')))
            }
            _ => panic!("expected to fail"),
        };
        let lexer = Lexer::new("let".to_string());
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Err(errors) => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0], ParserError::WhereIsEverybody)
            }
            _ => panic!("expected to fail"),
        };
        let lexer = Lexer::new("let 13".to_string());
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Err(errors) => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0], ParserError::IDontWantThis(Token::Integer(13)))
            }
            _ => panic!("expected to fail"),
        };
        let lexer = Lexer::new("let what".to_string());
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Err(errors) => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0], ParserError::WhyNothingIfIWantThis(Token::Equals))
            }
            _ => panic!("expected to fail"),
        };
    }

    fn assert_identifier(node: Box<dyn Expression>, expected: &str) {
        if let Some(ident) = node.as_any().downcast_ref::<Identifier>() {
            assert_eq!(ident.token, Token::Identifier(expected.to_string()));
        } else {
            panic!("expected Identifier node");
        }
    }

    fn assert_return_statement(stmt: Box<dyn Statement>) {
        if let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
            assert_eq!(return_stmt.token, Token::Return);
        } else {
            panic!("expected ReturnStatement node");
        }
    }

    fn assert_let_statement(stmt: Box<dyn Statement>, expected_identifier: &str) {
        if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
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
