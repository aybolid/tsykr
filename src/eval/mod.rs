mod env;
mod object;

pub use env::*;
pub use object::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {}

pub trait Eval {
    fn eval(&self, env: &ExecEnvironment) -> Result<Box<dyn Object>, EvalError>;
}
