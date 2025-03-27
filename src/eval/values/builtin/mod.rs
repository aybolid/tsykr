use std::{fmt::Debug, rc::Rc};

use crate::eval::EvalError;

use super::Value;

mod print;

pub use print::*;

pub trait BuiltinFunction
where
    Self: Debug,
{
    fn get_identifier(&self) -> String;
    fn call(&self, args: Vec<Rc<Value>>) -> Result<Rc<Value>, EvalError>;
}

impl PartialEq for Box<dyn BuiltinFunction> {
    fn eq(&self, _other: &Self) -> bool {
        // Define how BuiltinFunction objects should be compared
        false // Placeholder; change based on actual logic
    }
}
