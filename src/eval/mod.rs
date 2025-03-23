mod env;
mod object;

use std::sync::Arc;

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
}

pub trait Eval {
    fn eval(&self, env: &mut ExecEnvironment) -> Result<Option<Arc<Object>>, EvalError>;
}
