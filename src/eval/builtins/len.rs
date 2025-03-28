use std::rc::Rc;

use crate::{
    eval::{EvalError, Value},
    lexer::Position,
};

pub fn length(args: Vec<Rc<Value>>, pos: Position) -> Result<Rc<Value>, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::BuiltinWrongNumberOfArguments(
            "len".to_string(),
            1,
            args.len(),
            pos,
        ));
    }

    if !args[0].is_string() {
        return Err(EvalError::BuiltinWrongArgumentType(
            "len".to_string(),
            "String or Array".to_string(),
            args[0].to_string(),
            pos,
        ));
    }

    match &*args[0] {
        Value::String(str) => Ok(Value::new_integer(str.len() as i64)),
        _ => unreachable!(),
    }
}
