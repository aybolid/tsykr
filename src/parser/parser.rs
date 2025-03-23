use thiserror::Error;

use crate::lexer::{Lexer, Token, TokenKind};

use super::{
    precedence::Precedence, Block, Boolean, Expression, ExpressionStatement, Float, FunctionCall,
    FunctionDeclaration, FunctionExpression, Identifier, Infixed, Integer, LetStatement, Prefixed,
    Program, ReturnStatement, Statement,
};

#[derive(Debug, PartialEq, Error)]
pub enum ParserError {
    #[error("Unexpected token: {0}")]
    InvalidToken(Token),
    #[error("Unexpected token: wanted {expected}; got {actual}")]
    UnexpectedToken { expected: TokenKind, actual: Token },
    #[error("Unexpected end of input: expected {0}")]
    UnexpectedEOFWithExpectation(TokenKind),
    #[error("Unexpected end of input")]
    UnexpectedEOF,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    /// Creates a new parser instance.
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

    /// Parses a program.
    /// Returns a `Result` containing a `Program` or a vector of `ParserError`s.
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
    fn parse_statement(&mut self) -> Result<Box<Statement>, ParserError> {
        if let Some(ref token) = self.current_token {
            match token.kind {
                TokenKind::Let => Ok(Box::new(self.parse_let_statement()?)),
                TokenKind::Return => Ok(Box::new(self.parse_return_statement()?)),
                TokenKind::Function
                    if self.peek_token.is_some()
                        && matches!(
                            self.peek_token.as_ref().expect("checked before").kind,
                            TokenKind::Identifier(_)
                        ) =>
                {
                    Ok(Box::new(self.parse_function_declaration_statement()?))
                }
                TokenKind::LeftCurly => Ok(Box::new(self.parse_block_statement()?)),
                TokenKind::ILLEGAL(_) => Err(ParserError::InvalidToken(token.clone())),
                _ => Ok(Box::new(self.parse_expression_statement()?)),
            }
        } else {
            Err(ParserError::UnexpectedEOF)
        }
    }

    /// Parses a function declaration statement.
    fn parse_function_declaration_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::Function)?;
        let fn_token = self.current_token.take().expect("checked before");
        self.next_token();

        let identifier = match self.parse_identifier() {
            Ok(identifier) => match identifier {
                Expression::IDENTIFIER(identifier) => identifier,
                _ => unreachable!(),
            },
            Err(err) => return Err(err),
        };
        self.expect_token_kind(&self.peek_token, TokenKind::LeftParen)?;
        self.next_token();

        let params = self.parse_function_parameters()?;
        self.next_token();

        let body = match self.parse_block_statement() {
            Ok(block) => match block {
                Statement::BLOCK(block) => block,
                _ => unreachable!(),
            },
            Err(err) => return Err(err),
        };

        Ok(Statement::FUNCTION(FunctionDeclaration::new(
            fn_token, identifier, params, body,
        )))
    }

    /// Parses a block statement.
    fn parse_block_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::LeftCurly)?;
        let block_start_token = self.current_token.take().expect("checked before");
        self.next_token();

        let mut statements = vec![];

        while self
            .current_token
            .as_ref()
            .map(|t| t.kind != TokenKind::RightCurly)
            .unwrap_or(false)
        {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        self.expect_token_kind(&self.current_token, TokenKind::RightCurly)?;

        Ok(Statement::BLOCK(Block::new(block_start_token, statements)))
    }

    /// Parses a function parameters.
    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::LeftParen)?;
        let mut params = vec![];

        if self
            .peek_token
            .as_ref()
            .map(|t| t.kind == TokenKind::RightParen)
            .unwrap_or(false)
        {
            self.next_token();
            return Ok(params);
        }

        self.next_token();

        let ident = match self.parse_identifier() {
            Ok(identifier) => match identifier {
                Expression::IDENTIFIER(identifier) => identifier,
                _ => unreachable!(),
            },
            Err(err) => return Err(err),
        };
        params.push(ident);

        while self
            .peek_token
            .as_ref()
            .map(|t| t.kind == TokenKind::Comma)
            .unwrap_or(false)
        {
            self.next_token();
            self.next_token();
            let ident = match self.parse_identifier() {
                Ok(identifier) => match identifier {
                    Expression::IDENTIFIER(identifier) => identifier,
                    _ => unreachable!(),
                },
                Err(err) => return Err(err),
            };
            params.push(ident);
        }

        self.expect_token_kind(&self.peek_token, TokenKind::RightParen)?;
        self.next_token();

        Ok(params)
    }

    /// Parses an expression statement
    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        if self.current_token.is_none() {
            return Err(ParserError::UnexpectedEOF);
        }

        // clone token as it will be consumed by the expression parser
        let expr_token = self.current_token.clone().expect("checked before");
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self
            .peek_token
            .as_ref()
            .map(|t| t.kind == TokenKind::SemiColon)
            .unwrap_or(false)
        {
            self.next_token();
        }

        Ok(Statement::EXPR(ExpressionStatement::new(expr_token, expr)))
    }

    /// Parses an expression.
    fn parse_expression(&mut self, precedence: Precedence) -> Result<Box<Expression>, ParserError> {
        if self.current_token.is_none() {
            return Err(ParserError::UnexpectedEOF);
        }

        let mut expr: Box<Expression> =
            match self.current_token.as_ref().expect("checked before").kind {
                TokenKind::Identifier(_) => Box::new(self.parse_identifier()?),
                TokenKind::Integer(_) => Box::new(self.parse_integer()?),
                TokenKind::Float(_) => Box::new(self.parse_float()?),
                TokenKind::True | TokenKind::False => Box::new(self.parse_boolean()?),
                TokenKind::Bang | TokenKind::Minus => Box::new(self.parse_prefixed_expression()?),
                TokenKind::Function => Box::new(self.parse_function_expression()?),

                TokenKind::LeftParen => self.parse_grouped_expression()?,

                _ => {
                    return Err(ParserError::InvalidToken(
                        self.current_token.clone().unwrap(),
                    ))
                }
            };

        while self.peek_token.is_some()
            && self.peek_token.as_ref().unwrap().kind != TokenKind::SemiColon
            && precedence < Precedence::from_token(self.peek_token.clone().expect("checked before"))
        {
            match self.peek_token.as_ref().unwrap().kind {
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Slash
                | TokenKind::Asterisk
                | TokenKind::LessThan
                | TokenKind::LessThanEquals
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEquals
                | TokenKind::EqualsEquals
                | TokenKind::BangEquals => {
                    self.next_token();
                    expr = Box::new(self.parse_infixed_expression(expr)?);
                }
                TokenKind::LeftParen => {
                    self.next_token();
                    expr = Box::new(self.parse_function_call(expr)?);
                }
                _ => return Ok(expr),
            };
        }

        Ok(expr)
    }

    /// Parses a function call expression.
    fn parse_function_call(&mut self, callee: Box<Expression>) -> Result<Expression, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::LeftParen)?;
        let call_token = self.current_token.take().expect("checked before");
        let arguments = self.parse_expression_list(TokenKind::RightParen)?;

        Ok(Expression::CALL(FunctionCall::new(
            call_token, callee, arguments,
        )))
    }

    /// Parses a list of expressions separated by commas.
    /// `end_token_kind` is the token kind that marks the end of the list.
    fn parse_expression_list(
        &mut self,
        end_token_kind: TokenKind,
    ) -> Result<Vec<Box<Expression>>, ParserError> {
        let mut list = vec![];

        if self
            .peek_token
            .as_ref()
            .map(|t| t.kind == end_token_kind)
            .unwrap_or(false)
        {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self
            .peek_token
            .as_ref()
            .map(|t| t.kind == TokenKind::Comma)
            .unwrap_or(false)
        {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        self.expect_token_kind(&self.peek_token, end_token_kind)?;
        self.next_token();

        Ok(list)
    }

    fn parse_function_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::Function)?;
        let fn_token = self.current_token.take().expect("checked before");
        self.next_token();

        self.expect_token_kind(&self.current_token, TokenKind::LeftParen)?;
        let params = self.parse_function_parameters()?;
        self.next_token();

        let body = match self.parse_block_statement() {
            Ok(body) => match body {
                Statement::BLOCK(block) => block,
                _ => unreachable!(),
            },
            Err(err) => return Err(err),
        };

        Ok(Expression::FUNCTION(FunctionExpression::new(
            fn_token, params, body,
        )))
    }

    /// Parses a grouped expression (wrapped in parentheses).
    fn parse_grouped_expression(&mut self) -> Result<Box<Expression>, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::LeftParen)?;
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest)?;
        self.expect_token_kind(&self.peek_token, TokenKind::RightParen)?;
        self.next_token();
        Ok(expr)
    }

    /// Parses an infixed expression.
    fn parse_infixed_expression(
        &mut self,
        left: Box<Expression>,
    ) -> Result<Expression, ParserError> {
        let op_token = self.current_token.take().expect("checked before");
        self.next_token();
        let right = self.parse_expression(Precedence::from_token(op_token.clone()))?;
        Ok(Expression::INFIXED(Infixed::new(op_token, left, right)))
    }

    /// Parses a prefixed expression.
    fn parse_prefixed_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Bang | TokenKind::Minus)
        })?;
        let op_token = self.current_token.take().expect("checked before");
        self.next_token();
        Ok(Expression::PREFIXED(Prefixed::new(
            op_token,
            self.parse_expression(Precedence::Prefix)?,
        )))
    }

    /// Parses a boolean literal.
    ///
    /// Takes the current token and returns a Boolean.
    fn parse_boolean(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::True | TokenKind::False)
        })?;
        Ok(Expression::BOOLEAN(Boolean::new(
            self.current_token.take().expect("checked before"),
        )))
    }

    /// Parses a float literal.
    ///
    /// Takes the current token and returns a Float.
    fn parse_float(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Float(_))
        })?;
        Ok(Expression::FLOAT(Float::new(
            self.current_token.take().expect("checked before"),
        )))
    }

    /// Parses an integer literal.
    ///
    /// Takes the current token and returns an Integer.
    fn parse_integer(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Integer(_))
        })?;
        Ok(Expression::INTEGER(Integer::new(
            self.current_token.take().expect("checked before"),
        )))
    }

    /// Parses an identifier.
    ///
    /// Takes the current token and returns an Identifier.
    fn parse_identifier(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Identifier(_))
        })?;
        Ok(Expression::IDENTIFIER(Identifier::new(
            self.current_token.take().expect("checked before"),
        )))
    }

    /// Parses a return statement
    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::Return)?;
        let return_token = self.current_token.take().expect("checked before");
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self
            .peek_token
            .as_ref()
            .map(|t| t.kind == TokenKind::SemiColon)
            .unwrap_or(false)
        {
            self.next_token();
        }

        Ok(Statement::RETURN(ReturnStatement::new(return_token, value)))
    }

    /// Parses a let statement
    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::Let)?;
        let let_token = self.current_token.take().expect("checked before");
        self.next_token();

        let identifier = match self.parse_identifier() {
            Ok(identifier) => match identifier {
                Expression::IDENTIFIER(identifier) => identifier,
                _ => unreachable!(),
            },
            Err(err) => return Err(err),
        };
        self.next_token();

        self.expect_token_kind(&self.current_token, TokenKind::Equals)?;
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self
            .peek_token
            .as_ref()
            .map(|t| t.kind == TokenKind::SemiColon)
            .unwrap_or(false)
        {
            self.next_token();
        }

        Ok(Statement::LET(LetStatement::new(
            let_token, identifier, value,
        )))
    }

    /// Populates the current token and the peek token.
    /// Advances the lexer to the next token.
    fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
    }

    /// Checks if the current token matches the expected token kind.
    ///
    /// Generally, it's better to use this method instead of `expect_token_fn` as it produces a more descriptive error message.
    fn expect_token_kind(
        &self,
        to_check: &Option<Token>,
        expected_token_kind: TokenKind,
    ) -> Result<(), ParserError> {
        match to_check {
            Some(token) if token.kind == expected_token_kind => Ok(()),
            Some(token) => Err(ParserError::UnexpectedToken {
                expected: expected_token_kind,
                actual: token.clone(),
            }),
            None => Err(ParserError::UnexpectedEOFWithExpectation(
                expected_token_kind,
            )),
        }
    }

    /// Checks if the token matches the predicate.
    ///
    /// Use `expect_token_kind` if equality check is all you need.
    fn expect_token_fn<F>(&self, to_check: &Option<Token>, predicate: F) -> Result<(), ParserError>
    where
        F: Fn(&Token) -> bool,
    {
        match to_check {
            Some(token) if predicate(token) => Ok(()),
            Some(token) => Err(ParserError::InvalidToken(token.clone())),
            None => Err(ParserError::UnexpectedEOF),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{lexer::Position, parser::Node};

//     use super::*;

//     #[test]
//     fn test_new_parser() {
//         let lexer = Lexer::new("let".to_string());
//         let parser = Parser::new(lexer);
//         assert_eq!(
//             parser.current_token,
//             Some(Token::new(TokenKind::Let, Position(1, 1)))
//         )
//     }

//     #[test]
//     fn test_next_token() {
//         let lexer = Lexer::new("let x = 5".to_string());
//         let mut parser = Parser::new(lexer);
//         parser.next_token();
//         assert_eq!(
//             parser.current_token,
//             Some(Token::new(
//                 TokenKind::Identifier("x".to_string()),
//                 Position(1, 5)
//             ))
//         );
//         parser.next_token();
//         assert_eq!(
//             parser.current_token,
//             Some(Token::new(TokenKind::Equals, Position(1, 7)))
//         );
//         parser.next_token();
//         assert_eq!(
//             parser.current_token,
//             Some(Token::new(TokenKind::Integer(5), Position(1, 9)))
//         );
//         parser.next_token();
//         assert_eq!(parser.current_token, None);
//     }

//     #[test]
//     fn test_expect_peek_token() {
//         let lexer = Lexer::new("let x = 5".to_string());
//         let mut parser = Parser::new(lexer);

//         println!("{parser:?}");

//         parser
//             .expect_token_fn(&parser.peek_token, |token| {
//                 matches!(token.kind, TokenKind::Identifier(_))
//             })
//             .unwrap();
//         assert_eq!(
//             parser.peek_token,
//             Some(Token::new(
//                 TokenKind::Identifier("x".to_string()),
//                 Position(1, 5)
//             ))
//         );
//         parser.next_token();

//         parser
//             .expect_token_kind(&parser.peek_token, TokenKind::Equals)
//             .unwrap();
//         assert_eq!(
//             parser.peek_token,
//             Some(Token::new(TokenKind::Equals, Position(1, 7)))
//         );
//         parser.next_token();

//         parser
//             .expect_token_fn(&parser.peek_token, |token| {
//                 matches!(token.kind, TokenKind::Integer(_))
//             })
//             .unwrap();
//         assert_eq!(
//             parser.peek_token,
//             Some(Token::new(TokenKind::Integer(5), Position(1, 9)))
//         );
//         if let Ok(()) = parser.expect_token_kind(&parser.peek_token, TokenKind::If) {
//             panic!("expect_next_token should fail at this point");
//         }
//     }

//     #[test]
//     fn test_parse_function_call_expression() {
//         let lexer = Lexer::new("(1, 2)".to_string());
//         let mut parser = Parser::new(lexer);
//         let call = parser
//             .parse_function_call(Box::new(Identifier::new(Token::new(
//                 TokenKind::Identifier("foo".to_string()),
//                 Position(1, 1),
//             ))))
//             .unwrap();

//         assert!(call.as_any().is::<FunctionCall>());
//         assert_eq!(call.function.to_string(), "foo");
//         assert_eq!(call.arguments.len(), 2);
//         assert_eq!(call.arguments[0].to_string(), "1");
//         assert_eq!(call.arguments[1].to_string(), "2");
//     }

//     #[test]
//     fn test_parse_function_expression() {
//         let lexer = Lexer::new("fn (a) { return a }".to_string());
//         let mut parser = Parser::new(lexer);
//         let func = parser.parse_function_expression().unwrap();

//         assert!(func.as_any().is::<FunctionExpression>());
//         assert_eq!(func.token, Token::new(TokenKind::Function, Position(1, 1)));
//         assert_eq!(func.parameters.len(), 1);
//         assert_eq!(func.parameters[0].to_string(), "a");
//         assert_eq!(func.body.statements.len(), 1);
//         assert_eq!(func.body.to_string(), "{\n  return a\n}")
//     }

//     #[test]
//     fn test_parse_function_declaration_statement() {
//         let lexer = Lexer::new("fn foo(x, y) { return x + y; }".to_string());
//         let mut parser = Parser::new(lexer);
//         let func = parser.parse_function_declaration_statement().unwrap();

//         assert_eq!(func.token, Token::new(TokenKind::Function, Position(1, 1)));
//         assert_eq!(func.identifier.to_string(), "foo");
//         assert_eq!(func.parameters.len(), 2);
//         assert_eq!(func.parameters[0].to_string(), "x");
//         assert_eq!(func.parameters[1].to_string(), "y");
//         assert_eq!(func.body.statements.len(), 1);
//         assert_eq!(func.body.to_string(), "{\n  return (x+y)\n}")
//     }

//     #[test]
//     fn test_parse_function_parameters() {
//         let lexer = Lexer::new("(a, b)".to_string());
//         let mut parser = Parser::new(lexer);
//         let params = parser.parse_function_parameters().unwrap();

//         assert_eq!(params.len(), 2);
//         assert_eq!(params[0].to_string(), "a");
//         assert_eq!(params[1].to_string(), "b");
//     }

//     #[test]
//     fn test_parse_block_statement() {
//         let lexer = Lexer::new("{ 5; ident; }".to_string());
//         let mut parser = Parser::new(lexer);
//         let block = parser.parse_block_statement().unwrap();

//         assert_eq!(block.statements.len(), 2);

//         let int_expr = block.statements[0]
//             .as_any()
//             .downcast_ref::<ExpressionStatement>()
//             .unwrap()
//             .expression
//             .as_any()
//             .downcast_ref::<Integer>()
//             .unwrap();
//         assert_eq!(int_expr.to_string(), "5");

//         let ident_expr = block.statements[1]
//             .as_any()
//             .downcast_ref::<ExpressionStatement>()
//             .unwrap()
//             .expression
//             .as_any()
//             .downcast_ref::<Identifier>()
//             .unwrap();
//         assert_eq!(ident_expr.to_string(), "ident");
//     }

//     #[test]
//     fn test_parse_grouped() {
//         let lexer = Lexer::new("let x = (2 + 2) * 2;".to_string());
//         let mut parser = Parser::new(lexer);
//         let program = parser.parse().unwrap();
//         assert_eq!(program.to_string(), "let x = ((2+2)*2)");
//     }

//     #[test]
//     fn test_parse_infixed() {
//         let lexer = Lexer::new("6 + 2".to_string());
//         let mut parser = Parser::new(lexer);

//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             if let Some(infix_expr) = expr_stmt.expression.as_any().downcast_ref::<Infixed>() {
//                 assert_eq!(infix_expr.left.token_literal(), "6");
//                 assert_eq!(infix_expr.op, Token::new(TokenKind::Plus, Position(1, 3)));
//                 assert_eq!(infix_expr.right.token_literal(), "2");
//             } else {
//                 panic!("expected Infix node");
//             }
//         } else {
//             panic!("expected ExpressionStatement node");
//         }
//     }

//     #[test]
//     fn test_parse_prefixed() {
//         let lexer = Lexer::new("-5 !true".to_string());
//         let mut parser = Parser::new(lexer);

//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             if let Some(prefix_expr) = expr_stmt.expression.as_any().downcast_ref::<Prefixed>() {
//                 assert_eq!(prefix_expr.op, Token::new(TokenKind::Minus, Position(1, 1)));
//                 assert_eq!(prefix_expr.right.token_literal(), "5");
//             } else {
//                 panic!("expected Prefixed node");
//             }
//         } else {
//             panic!("expected ExpressionStatement node");
//         }

//         parser.next_token();
//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             if let Some(prefix_expr) = expr_stmt.expression.as_any().downcast_ref::<Prefixed>() {
//                 assert_eq!(prefix_expr.op, Token::new(TokenKind::Bang, Position(1, 4)));
//                 assert_eq!(prefix_expr.right.token_literal(), "true");
//             } else {
//                 panic!("expected Prefixed node");
//             }
//         } else {
//             panic!("expected ExpressionStatement node");
//         }
//     }

//     #[test]
//     fn test_parse_boolean() {
//         let lexer = Lexer::new("true false".to_string());
//         let mut parser = Parser::new(lexer);

//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "true");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }

//         parser.next_token();
//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "false");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }
//     }

//     #[test]
//     fn test_parse_float() {
//         let lexer = Lexer::new("3.14 32.00878".to_string());
//         let mut parser = Parser::new(lexer);

//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "3.14");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }

//         parser.next_token();
//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "32.00878");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }
//     }

//     #[test]
//     fn test_parse_integer() {
//         let lexer = Lexer::new("5 009".to_string());
//         let mut parser = Parser::new(lexer);

//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "5");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }

//         parser.next_token();
//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "9");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }
//     }

//     #[test]
//     fn test_parse_identifier() {
//         let lexer = Lexer::new("name variable".to_string());
//         let mut parser = Parser::new(lexer);

//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "name");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }

//         parser.next_token();
//         let stmt = parser.parse_statement().unwrap();
//         if let Some(expr_stmt) = stmt.as_any().downcast_ref::<ExpressionStatement>() {
//             assert_eq!(expr_stmt.expression.token_literal(), "variable");
//         } else {
//             panic!("expected ExpressionStatement node");
//         }
//     }

//     #[test]
//     fn test_parse_let_statement() {
//         let lexer = Lexer::new("let x = 5;".to_string());
//         let mut parser = Parser::new(lexer);
//         let stmt = parser.parse_statement().unwrap();
//         if let Some(let_stmt) = stmt.as_any().downcast_ref::<LetStatement>() {
//             assert_eq!(let_stmt.token.kind, TokenKind::Let);
//             assert_eq!(let_stmt.value.to_string(), "5")
//         } else {
//             panic!("expected LetStatement node");
//         }
//     }

//     #[test]
//     fn test_parse_return_statement() {
//         let lexer = Lexer::new("return 5;".to_string());
//         let mut parser = Parser::new(lexer);
//         let stmt = parser.parse_statement().unwrap();
//         if let Some(return_stmt) = stmt.as_any().downcast_ref::<ReturnStatement>() {
//             assert_eq!(return_stmt.token.kind, TokenKind::Return);
//             assert_eq!(return_stmt.value.to_string(), "5")
//         } else {
//             panic!("expected ReturnStatement node");
//         }
//     }

//     #[test]
//     fn test_parser_errors() {
//         let lexer = Lexer::new("№".to_string());
//         let mut parser = Parser::new(lexer);
//         match parser.parse() {
//             Err(errors) => {
//                 assert_eq!(errors.len(), 1);
//                 assert_eq!(
//                     errors[0],
//                     ParserError::InvalidToken(Token::new(TokenKind::ILLEGAL('№'), Position(1, 1)))
//                 )
//             }
//             _ => panic!("expected to fail"),
//         };

//         let lexer = Lexer::new("let".to_string());
//         let mut parser = Parser::new(lexer);
//         match parser.parse() {
//             Err(errors) => {
//                 assert_eq!(errors.len(), 1);
//                 assert_eq!(errors[0], ParserError::UnexpectedEOF)
//             }
//             _ => panic!("expected to fail"),
//         };

//         let lexer = Lexer::new("let 13".to_string());
//         let mut parser = Parser::new(lexer);
//         match parser.parse() {
//             Err(errors) => {
//                 assert_eq!(errors.len(), 1);
//                 assert_eq!(
//                     errors[0],
//                     ParserError::InvalidToken(Token::new(TokenKind::Integer(13), Position(1, 5)))
//                 )
//             }
//             _ => panic!("expected to fail"),
//         };

//         let lexer = Lexer::new("let what".to_string());
//         let mut parser = Parser::new(lexer);
//         match parser.parse() {
//             Err(errors) => {
//                 assert_eq!(errors.len(), 1);
//                 assert_eq!(
//                     errors[0],
//                     ParserError::UnexpectedEOFWithExpectation(TokenKind::Equals)
//                 )
//             }
//             _ => panic!("expected to fail"),
//         };
//     }
// }
