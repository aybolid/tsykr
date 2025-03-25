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
}

pub trait Eval {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError>;
}
