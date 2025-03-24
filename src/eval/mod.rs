mod env;
mod object;

use std::{cell::RefCell, rc::Rc};

pub use env::*;
pub use object::*;

use thiserror::Error;

use crate::lexer::Position;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Unknown identifier: {0} - {1}")]
    UnknownIdentifier(String, Position),
    #[error("Invalid prefix operation: {operator} on {operand} - {position}")]
    InvalidPrefixOperation {
        operator: String,
        operand: String,
        position: Position,
    },
    #[error("Invalid infix operation: {left} {operator} {right} - {position}")]
    InvalidInfixOperation {
        operator: String,
        left: String,
        right: String,
        position: Position,
    },
    #[error("Not a function: {0} - {1}")]
    NotAFunction(String, Position),
}

pub trait Eval {
    fn eval(&self, env: Rc<RefCell<ExecEnvironment>>) -> Result<Option<Rc<Object>>, EvalError>;
}
