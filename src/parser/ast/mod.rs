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
