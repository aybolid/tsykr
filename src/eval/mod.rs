mod exec_env;
mod values;

use std::{cell::RefCell, rc::Rc};

pub use exec_env::*;
use thiserror::Error;
pub use values::*;

use crate::lexer::Position;

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("Tried to store void value: {0}")]
    TriedToStoreVoid(Position),
    #[error("Not defined: {0} at {1}")]
    NotDefined(String, Position),
    #[error("Invalid prefix operation: {0}{1} at {2}")]
    InvalidPrefixOperation(String, String, Position),
    #[error("Invalid infix operation: {0}{1}{2} at {3}")]
    InvalidInfixOperation(String, String, String, Position),
    #[error("Division by zero: {0}/{1} at {2}")]
    DivisionByZero(String, String, Position),
}

pub trait Eval {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError>;
}
