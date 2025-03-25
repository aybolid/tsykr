use thiserror::Error;

use crate::lexer::{Lexer, Token, TokenKind};

use super::{precedence::Precedence, Expression, Identifier, Program, Statement};

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
                Expression::IdentExpr(identifier) => identifier,
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
                Statement::BlockStatement(block) => block,
                _ => unreachable!(),
            },
            Err(err) => return Err(err),
        };

        Ok(Statement::new_fn(fn_token, identifier, params, body))
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

        Ok(Statement::new_block(block_start_token, statements))
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
                Expression::IdentExpr(identifier) => identifier,
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
                    Expression::IdentExpr(identifier) => identifier,
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

        Ok(Statement::new_expr(expr_token, expr))
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

        Ok(Expression::new_call(call_token, callee, arguments))
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
                Statement::BlockStatement(block) => block,
                _ => unreachable!(),
            },
            Err(err) => return Err(err),
        };

        Ok(Expression::new_fn(fn_token, params, body))
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
        Ok(Expression::new_infixed(op_token, left, right))
    }

    /// Parses a prefixed expression.
    fn parse_prefixed_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Bang | TokenKind::Minus)
        })?;
        let op_token = self.current_token.take().expect("checked before");
        self.next_token();
        Ok(Expression::new_prefixed(
            op_token,
            self.parse_expression(Precedence::Prefix)?,
        ))
    }

    /// Parses a boolean literal.
    ///
    /// Takes the current token and returns a Boolean.
    fn parse_boolean(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::True | TokenKind::False)
        })?;
        Ok(Expression::new_boolean(
            self.current_token.take().expect("checked before"),
        ))
    }

    /// Parses a float literal.
    ///
    /// Takes the current token and returns a Float.
    fn parse_float(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Float(_))
        })?;
        Ok(Expression::new_float(
            self.current_token.take().expect("checked before"),
        ))
    }

    /// Parses an integer literal.
    ///
    /// Takes the current token and returns an Integer.
    fn parse_integer(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Integer(_))
        })?;
        Ok(Expression::new_int(
            self.current_token.take().expect("checked before"),
        ))
    }

    /// Parses an identifier.
    ///
    /// Takes the current token and returns an Identifier.
    fn parse_identifier(&mut self) -> Result<Expression, ParserError> {
        self.expect_token_fn(&self.current_token, |t| {
            matches!(t.kind, TokenKind::Identifier(_))
        })?;
        Ok(Expression::new_ident(
            self.current_token.take().expect("checked before"),
        ))
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

        Ok(Statement::new_return(return_token, value))
    }

    /// Parses a let statement
    fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_token_kind(&self.current_token, TokenKind::Let)?;
        let let_token = self.current_token.take().expect("checked before");
        self.next_token();

        let identifier = match self.parse_identifier() {
            Ok(identifier) => match identifier {
                Expression::IdentExpr(identifier) => identifier,
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

        Ok(Statement::new_let(let_token, identifier, value))
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

#[cfg(test)]
mod parser_tests {
    use super::*;

    /// Helper function to create a simple lexer from a string input
    fn create_lexer(input: &str) -> Lexer {
        Lexer::new(input.to_string())
    }

    /// Helper function to parse and extract the first statement
    fn parse_first_statement(input: &str) -> Result<Box<Statement>, Vec<ParserError>> {
        let lexer = create_lexer(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse()?;
        Ok(program.statements.into_iter().next().unwrap())
    }

    #[test]
    fn test_let_statement_parsing() {
        let input = "let x = 5;";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Let(let_stmt) => {
                assert_eq!(let_stmt.identifier.token.literal(), "x");

                match let_stmt.value.as_ref() {
                    Expression::IntExpr(int_expr) => {
                        assert_eq!(int_expr.token.literal(), "5");
                    }
                    _ => panic!("Expected integer expression"),
                }
            }
            _ => panic!("Expected let statement"),
        }
    }

    #[test]
    fn test_return_statement_parsing() {
        let input = "return 5;";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Return(return_stmt) => match return_stmt.value.as_ref() {
                Expression::IntExpr(int_expr) => {
                    assert_eq!(int_expr.token.literal(), "5");
                }
                _ => panic!("Expected integer expression"),
            },
            _ => panic!("Expected return statement"),
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Expr(expr_stmt) => match expr_stmt.expression.as_ref() {
                Expression::IdentExpr(ident) => {
                    assert_eq!(ident.token.literal(), "foobar");
                }
                _ => panic!("Expected identifier expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Expr(expr_stmt) => match expr_stmt.expression.as_ref() {
                Expression::IntExpr(int_expr) => {
                    assert_eq!(int_expr.token.literal(), "5");
                }
                _ => panic!("Expected integer expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_float_literal_expression() {
        let input = "5.5;";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Expr(expr_stmt) => match expr_stmt.expression.as_ref() {
                Expression::FloatExpr(float_expr) => {
                    assert_eq!(float_expr.token.literal(), "5.5");
                }
                _ => panic!("Expected float expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_boolean_expressions() {
        let inputs = vec!["true;", "false;"];

        for input in inputs {
            let statement = parse_first_statement(input).unwrap();

            match *statement {
                Statement::Expr(expr_stmt) => match expr_stmt.expression.as_ref() {
                    Expression::BooleanExpr(bool_expr) => {
                        assert_eq!(bool_expr.token.literal(), input.trim_end_matches(';'));
                    }
                    _ => panic!("Expected boolean expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
    }

    #[test]
    fn test_prefix_expressions() {
        let inputs = vec![
            ("!5;", TokenKind::Bang, "5"),
            ("-15;", TokenKind::Minus, "15"),
        ];

        for (input, expected_prefix, expected_value) in inputs {
            let statement = parse_first_statement(input).unwrap();

            match *statement {
                Statement::Expr(expr_stmt) => match expr_stmt.expression.as_ref() {
                    Expression::PrefixedExpr(prefix_expr) => {
                        assert_eq!(prefix_expr.op.kind, expected_prefix);
                        match prefix_expr.right.as_ref() {
                            Expression::IntExpr(int_expr) => {
                                assert_eq!(int_expr.token.literal(), expected_value);
                            }
                            _ => panic!("Expected integer expression"),
                        }
                    }
                    _ => panic!("Expected prefixed expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
    }

    #[test]
    fn test_infix_expressions() {
        let inputs = vec![
            ("5 + 5;", "5", TokenKind::Plus, "5"),
            ("5 - 5;", "5", TokenKind::Minus, "5"),
            ("5 * 5;", "5", TokenKind::Asterisk, "5"),
            ("5 / 5;", "5", TokenKind::Slash, "5"),
            ("5 > 5;", "5", TokenKind::GreaterThan, "5"),
            ("5 < 5;", "5", TokenKind::LessThan, "5"),
            ("5 == 5;", "5", TokenKind::EqualsEquals, "5"),
            ("5 != 5;", "5", TokenKind::BangEquals, "5"),
        ];

        for (input, expected_left, expected_op, expected_right) in inputs {
            let statement = parse_first_statement(input).unwrap();

            match *statement {
                Statement::Expr(expr_stmt) => match expr_stmt.expression.as_ref() {
                    Expression::InfixedExpr(infix_expr) => {
                        assert_eq!(infix_expr.op.kind, expected_op);

                        match infix_expr.left.as_ref() {
                            Expression::IntExpr(left_expr) => {
                                assert_eq!(left_expr.token.literal(), expected_left);
                            }
                            _ => panic!("Expected left integer expression"),
                        }

                        match infix_expr.right.as_ref() {
                            Expression::IntExpr(right_expr) => {
                                assert_eq!(right_expr.token.literal(), expected_right);
                            }
                            _ => panic!("Expected right integer expression"),
                        }
                    }
                    _ => panic!("Expected infixed expression"),
                },
                _ => panic!("Expected expression statement"),
            }
        }
    }

    #[test]
    fn test_function_declaration() {
        let input = "fn add(x, y) { x + y; }";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Fn(func_decl) => {
                assert_eq!(func_decl.identifier.token.literal(), "add");
                assert_eq!(func_decl.parameters.len(), 2);
                assert_eq!(func_decl.parameters[0].token.literal(), "x");
                assert_eq!(func_decl.parameters[1].token.literal(), "y");

                // Verify body statements
                assert_eq!(func_decl.body.statements.len(), 1);
            }
            _ => panic!("Expected function declaration"),
        }
    }

    #[test]
    fn test_function_expression() {
        let input = "fn(x, y) { x + y; };";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Expr(expr_stmt) => {
                match expr_stmt.expression.as_ref() {
                    Expression::FnExpr(func_expr) => {
                        assert_eq!(func_expr.parameters.len(), 2);
                        assert_eq!(func_expr.parameters[0].token.literal(), "x");
                        assert_eq!(func_expr.parameters[1].token.literal(), "y");

                        // Verify body statements
                        assert_eq!(func_expr.body.statements.len(), 1);
                    }
                    _ => panic!("Expected function expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_function_call() {
        let input = "add(1, 2 * 3);";
        let statement = parse_first_statement(input).unwrap();

        match *statement {
            Statement::Expr(expr_stmt) => {
                match expr_stmt.expression.as_ref() {
                    Expression::CallExpr(call_expr) => {
                        // Check callee
                        match call_expr.function.as_ref() {
                            Expression::IdentExpr(ident) => {
                                assert_eq!(ident.token.literal(), "add");
                            }
                            _ => panic!("Expected identifier as function"),
                        }

                        // Check arguments
                        assert_eq!(call_expr.arguments.len(), 2);

                        // First argument should be simple integer
                        match call_expr.arguments[0].as_ref() {
                            Expression::IntExpr(int_expr) => {
                                assert_eq!(int_expr.token.literal(), "1");
                            }
                            _ => panic!("Expected first argument to be an integer"),
                        }

                        // Second argument should be an infix expression
                        match call_expr.arguments[1].as_ref() {
                            Expression::InfixedExpr(infix_expr) => {
                                assert_eq!(infix_expr.op.kind, TokenKind::Asterisk);
                            }
                            _ => panic!("Expected second argument to be an infix expression"),
                        }
                    }
                    _ => panic!("Expected function call expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parsing_error_handling() {
        let inputs = vec![
            "let x = ;", // Missing value
            "let = 5;",  // Missing identifier
            "return;",   // Missing return value
        ];

        for input in inputs {
            let lexer = create_lexer(input);
            let mut parser = Parser::new(lexer);
            let result = parser.parse();

            assert!(
                result.is_err(),
                "Expected parsing to fail for input: {}",
                input
            );
        }
    }
}
