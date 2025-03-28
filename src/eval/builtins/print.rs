use std::rc::Rc;

use crate::{
    eval::{EvalError, Value, VOID},
    lexer::Position,
};

pub fn print(args: Vec<Rc<Value>>, _pos: Position) -> Result<Rc<Value>, EvalError> {
    print!(
        "{}",
        &args
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(VOID.rc())
}

pub fn println(args: Vec<Rc<Value>>, _pos: Position) -> Result<Rc<Value>, EvalError> {
    println!(
        "{}",
        &args
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(VOID.rc())
}
