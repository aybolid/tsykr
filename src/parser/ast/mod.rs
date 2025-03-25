mod expr_boolean;
mod expr_call;
mod expr_float;
mod expr_function;
mod expr_identifier;
mod expr_infixed;
mod expr_integer;
mod expr_prefixed;
mod program;
mod stmt_block;
mod stmt_expr;
mod stmt_function;
mod stmt_let;
mod stmt_return;

use std::fmt::Debug;

pub use expr_boolean::*;
pub use expr_call::*;
pub use expr_float::*;
pub use expr_function::*;
pub use expr_identifier::*;
pub use expr_infixed::*;
pub use expr_integer::*;
pub use expr_prefixed::*;
pub use program::*;
pub use stmt_block::*;
pub use stmt_expr::*;
pub use stmt_function::*;
pub use stmt_let::*;
pub use stmt_return::*;

use crate::lexer::Token;

pub trait Node
where
    Self: ToString,
    Self: Debug,
{
    /// Returns a token literal of the node.
    fn token_literal(&self) -> String;
    #[allow(unused)]
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(Block),
    ExpressionStatement(ExpressionStatement),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    FunctionDeclaration(FunctionDeclaration),
}

impl Statement {
    pub fn new_block(token: Token, statements: Vec<Box<Statement>>) -> Self {
        Statement::Block(Block::new(token, statements))
    }
    pub fn new_expression(token: Token, expr: Box<Expression>) -> Self {
        Statement::ExpressionStatement(ExpressionStatement::new(token, expr))
    }
    pub fn new_let(token: Token, name: Identifier, value: Box<Expression>) -> Self {
        Statement::LetStatement(LetStatement::new(token, name, value))
    }
    pub fn new_return(token: Token, value: Box<Expression>) -> Self {
        Statement::ReturnStatement(ReturnStatement::new(token, value))
    }
    pub fn new_function(
        token: Token,
        name: Identifier,
        params: Vec<Identifier>,
        body: Block,
    ) -> Self {
        Statement::FunctionDeclaration(FunctionDeclaration::new(token, name, params, body))
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Block(block) => block.token_literal(),
            Statement::ExpressionStatement(expr_stmt) => expr_stmt.token_literal(),
            Statement::LetStatement(let_stmt) => let_stmt.token_literal(),
            Statement::ReturnStatement(return_stmt) => return_stmt.token_literal(),
            Statement::FunctionDeclaration(func_decl) => func_decl.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        match self {
            Statement::Block(block) => block,
            Statement::ExpressionStatement(expr_stmt) => expr_stmt,
            Statement::LetStatement(let_stmt) => let_stmt,
            Statement::ReturnStatement(return_stmt) => return_stmt,
            Statement::FunctionDeclaration(func_decl) => func_decl,
        }
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Block(block) => block.to_string(),
            Statement::ExpressionStatement(expr_stmt) => expr_stmt.to_string(),
            Statement::LetStatement(let_stmt) => let_stmt.to_string(),
            Statement::ReturnStatement(return_stmt) => return_stmt.to_string(),
            Statement::FunctionDeclaration(func_decl) => func_decl.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Boolean(Boolean),
    FunctionCall(FunctionCall),
    Float(Float),
    Function(FunctionExpression),
    Identifier(Identifier),
    Infixed(Infixed),
    Integer(Integer),
    Prefixed(Prefixed),
}

impl Expression {
    pub fn new_boolean(token: Token) -> Self {
        Expression::Boolean(Boolean::new(token))
    }
    pub fn new_function_call(
        token: Token,
        func: Box<Expression>,
        args: Vec<Box<Expression>>,
    ) -> Self {
        Expression::FunctionCall(FunctionCall::new(token, func, args))
    }
    pub fn new_float(token: Token) -> Self {
        Expression::Float(Float::new(token))
    }
    pub fn new_integer(token: Token) -> Self {
        Expression::Integer(Integer::new(token))
    }
    pub fn new_function(token: Token, params: Vec<Identifier>, body: Block) -> Self {
        Expression::Function(FunctionExpression::new(token, params, body))
    }
    pub fn new_identifier(token: Token) -> Self {
        Expression::Identifier(Identifier::new(token))
    }
    pub fn new_prefixed(op_token: Token, right: Box<Expression>) -> Self {
        Expression::Prefixed(Prefixed::new(op_token, right))
    }
    pub fn new_infixed(op_token: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        Expression::Infixed(Infixed::new(op_token, left, right))
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Boolean(boolean) => boolean.token_literal(),
            Expression::FunctionCall(call) => call.token_literal(),
            Expression::Float(float) => float.token_literal(),
            Expression::Function(func) => func.token_literal(),
            Expression::Identifier(ident) => ident.token_literal(),
            Expression::Infixed(infix) => infix.token_literal(),
            Expression::Integer(int) => int.token_literal(),
            Expression::Prefixed(prefix) => prefix.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        match self {
            Expression::Boolean(boolean) => boolean,
            Expression::FunctionCall(call) => call,
            Expression::Float(float) => float,
            Expression::Function(func) => func,
            Expression::Identifier(ident) => ident,
            Expression::Infixed(infix) => infix,
            Expression::Integer(int) => int,
            Expression::Prefixed(prefix) => prefix,
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Boolean(boolean) => boolean.to_string(),
            Expression::FunctionCall(call) => call.to_string(),
            Expression::Float(float) => float.to_string(),
            Expression::Function(func) => func.to_string(),
            Expression::Identifier(ident) => ident.to_string(),
            Expression::Infixed(infix) => infix.to_string(),
            Expression::Integer(int) => int.to_string(),
            Expression::Prefixed(prefix) => prefix.to_string(),
        }
    }
}
