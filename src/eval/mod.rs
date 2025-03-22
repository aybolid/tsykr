mod object;

pub use object::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {}

pub trait Eval {
    fn eval(&self) -> Result<Box<dyn Object>, EvalError>;
}
