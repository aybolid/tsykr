use std::rc::Rc;

use crate::eval::{EvalError, Value, VOID};

use super::BuiltinFunction;

#[derive(Debug)]
pub struct PrintlnBuiltin;

impl BuiltinFunction for PrintlnBuiltin {
    fn get_identifier(&self) -> String {
        "println".to_string()
    }

    fn call(&self, args: Vec<Rc<Value>>) -> Result<Rc<Value>, EvalError> {
        println!(
            "{}",
            args.iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        Ok(VOID.rc())
    }
}
