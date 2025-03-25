mod exec_env;
mod values;

use std::{cell::RefCell, rc::Rc};

pub use exec_env::*;
use thiserror::Error;
pub use values::*;

#[derive(Debug, Error)]
pub enum EvalError {}

pub trait Eval {
    fn eval(&self, env: Rc<RefCell<ExecutionEnvironment>>) -> Result<Rc<Value>, EvalError>;
}
