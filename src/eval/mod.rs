mod builtins;
mod exec_env;
mod values;

use std::{cell::RefCell, rc::Rc};
use thiserror::Error;

pub use exec_env::*;
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
    #[error("Void as an function argument at {0}")]
    VoidValueAsArgument(Position),
    #[error("Not a function: {0}")]
    NotAFunction(Position),
    #[error("Wrong number of arguments: expected {0}, got {1} at {2}")]
    WrongNumberOfArguments(usize, usize, Position),
    #[error("Non-boolean condition: want true or false, got {0} at {1}")]
    NonBooleanCondition(String, Position),

    #[error("Wrong number of arguments: {0}: expected: {1}, got {2} at {3}")]
    BuiltinWrongNumberOfArguments(String, usize, usize, Position),
    #[error("Type mismatch: {0}: expected: {1}, got {2} at {3}")]
    BuiltinWrongArgumentType(String, String, String, Position),
}

pub trait Eval {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError>;
}
