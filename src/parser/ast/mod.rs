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
    BlockStatement(Block),
    Expr(ExpressionStatement),
    Let(LetStatement),
    Return(ReturnStatement),
    Fn(FunctionDeclaration),
}

impl Statement {
    pub fn new_block(token: Token, statements: Vec<Box<Statement>>) -> Self {
        Statement::BlockStatement(Block::new(token, statements))
    }
    pub fn new_expr(token: Token, expr: Box<Expression>) -> Self {
        Statement::Expr(ExpressionStatement::new(token, expr))
    }
    pub fn new_let(token: Token, name: Identifier, value: Box<Expression>) -> Self {
        Statement::Let(LetStatement::new(token, name, value))
    }
    pub fn new_return(token: Token, value: Box<Expression>) -> Self {
        Statement::Return(ReturnStatement::new(token, value))
    }
    pub fn new_fn(token: Token, name: Identifier, params: Vec<Identifier>, body: Block) -> Self {
        Statement::Fn(FunctionDeclaration::new(token, name, params, body))
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::BlockStatement(block) => block.token_literal(),
            Statement::Expr(expr_stmt) => expr_stmt.token_literal(),
            Statement::Let(let_stmt) => let_stmt.token_literal(),
            Statement::Return(return_stmt) => return_stmt.token_literal(),
            Statement::Fn(func_decl) => func_decl.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        match self {
            Statement::BlockStatement(block) => block,
            Statement::Expr(expr_stmt) => expr_stmt,
            Statement::Let(let_stmt) => let_stmt,
            Statement::Return(return_stmt) => return_stmt,
            Statement::Fn(func_decl) => func_decl,
        }
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::BlockStatement(block) => block.to_string(),
            Statement::Expr(expr_stmt) => expr_stmt.to_string(),
            Statement::Let(let_stmt) => let_stmt.to_string(),
            Statement::Return(return_stmt) => return_stmt.to_string(),
            Statement::Fn(func_decl) => func_decl.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BooleanExpr(Boolean),
    CallExpr(FunctionCall),
    FloatExpr(Float),
    FnExpr(FunctionExpression),
    IdentExpr(Identifier),
    InfixedExpr(Infixed),
    IntExpr(Integer),
    PrefixedExpr(Prefixed),
}

impl Expression {
    pub fn new_boolean(token: Token) -> Self {
        Expression::BooleanExpr(Boolean::new(token))
    }
    pub fn new_call(token: Token, func: Box<Expression>, args: Vec<Box<Expression>>) -> Self {
        Expression::CallExpr(FunctionCall::new(token, func, args))
    }
    pub fn new_float(token: Token) -> Self {
        Expression::FloatExpr(Float::new(token))
    }
    pub fn new_int(token: Token) -> Self {
        Expression::IntExpr(Integer::new(token))
    }
    pub fn new_fn(token: Token, params: Vec<Identifier>, body: Block) -> Self {
        Expression::FnExpr(FunctionExpression::new(token, params, body))
    }
    pub fn new_ident(token: Token) -> Self {
        Expression::IdentExpr(Identifier::new(token))
    }
    pub fn new_prefixed(op_token: Token, right: Box<Expression>) -> Self {
        Expression::PrefixedExpr(Prefixed::new(op_token, right))
    }
    pub fn new_infixed(op_token: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        Expression::InfixedExpr(Infixed::new(op_token, left, right))
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::BooleanExpr(boolean) => boolean.token_literal(),
            Expression::CallExpr(call) => call.token_literal(),
            Expression::FloatExpr(float) => float.token_literal(),
            Expression::FnExpr(func) => func.token_literal(),
            Expression::IdentExpr(ident) => ident.token_literal(),
            Expression::InfixedExpr(infix) => infix.token_literal(),
            Expression::IntExpr(int) => int.token_literal(),
            Expression::PrefixedExpr(prefix) => prefix.token_literal(),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        match self {
            Expression::BooleanExpr(boolean) => boolean,
            Expression::CallExpr(call) => call,
            Expression::FloatExpr(float) => float,
            Expression::FnExpr(func) => func,
            Expression::IdentExpr(ident) => ident,
            Expression::InfixedExpr(infix) => infix,
            Expression::IntExpr(int) => int,
            Expression::PrefixedExpr(prefix) => prefix,
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::BooleanExpr(boolean) => boolean.to_string(),
            Expression::CallExpr(call) => call.to_string(),
            Expression::FloatExpr(float) => float.to_string(),
            Expression::FnExpr(func) => func.to_string(),
            Expression::IdentExpr(ident) => ident.to_string(),
            Expression::InfixedExpr(infix) => infix.to_string(),
            Expression::IntExpr(int) => int.to_string(),
            Expression::PrefixedExpr(prefix) => prefix.to_string(),
        }
    }
}
