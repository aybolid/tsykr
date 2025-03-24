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

use std::{cell::RefCell, fmt::Debug, rc::Rc};

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

use crate::eval::{Eval, EvalError, ExecEnvironment, Object};

pub trait Node
where
    Self: ToString,
    Self: Debug,
{
    /// Returns a token literal of the node.
    #[allow(unused)]
    fn token_literal(&self) -> String;
    #[allow(unused)]
    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    BLOCK(Block),
    EXPR(ExpressionStatement),
    LET(LetStatement),
    RETURN(ReturnStatement),
    FUNCTION(FunctionDeclaration),
}

impl Statement {
    fn eval(&self, env: Rc<RefCell<ExecEnvironment>>) -> Result<Option<Rc<Object>>, EvalError> {
        match self {
            Statement::BLOCK(block) => block.eval(env),
            Statement::EXPR(expr_stmt) => expr_stmt.eval(env),
            Statement::LET(let_stmt) => let_stmt.eval(env),
            Statement::RETURN(return_stmt) => return_stmt.eval(env),
            Statement::FUNCTION(func_decl) => func_decl.eval(env),
        }
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::BLOCK(block) => block.to_string(),
            Statement::EXPR(expr_stmt) => expr_stmt.to_string(),
            Statement::LET(let_stmt) => let_stmt.to_string(),
            Statement::RETURN(return_stmt) => return_stmt.to_string(),
            Statement::FUNCTION(func_decl) => func_decl.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BOOLEAN(Boolean),
    CALL(FunctionCall),
    FLOAT(Float),
    FUNCTION(FunctionExpression),
    IDENTIFIER(Identifier),
    INFIXED(Infixed),
    INTEGER(Integer),
    PREFIXED(Prefixed),
}

impl Expression {
    pub fn eval(&self, env: Rc<RefCell<ExecEnvironment>>) -> Result<Option<Rc<Object>>, EvalError> {
        match self {
            Expression::BOOLEAN(boolean) => boolean.eval(env),
            Expression::CALL(call) => call.eval(env),
            Expression::FLOAT(float) => float.eval(env),
            Expression::FUNCTION(func) => func.eval(env),
            Expression::IDENTIFIER(ident) => ident.eval(env),
            Expression::INFIXED(infix) => infix.eval(env),
            Expression::INTEGER(int) => int.eval(env),
            Expression::PREFIXED(prefix) => prefix.eval(env),
        }
    }
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::BOOLEAN(boolean) => boolean.to_string(),
            Expression::CALL(call) => call.to_string(),
            Expression::FLOAT(float) => float.to_string(),
            Expression::FUNCTION(func) => func.to_string(),
            Expression::IDENTIFIER(ident) => ident.to_string(),
            Expression::INFIXED(infix) => infix.to_string(),
            Expression::INTEGER(int) => int.to_string(),
            Expression::PREFIXED(prefix) => prefix.to_string(),
        }
    }
}
