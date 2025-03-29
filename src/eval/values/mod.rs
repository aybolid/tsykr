use std::{cell::RefCell, rc::Rc};

use crate::{lexer::Position, parser::Block};

mod function;

pub use function::*;

use super::{EvalError, ExecutionEnvironment};

pub const VOID: Value = Value::VOID;
pub const TRUE: Value = Value::Boolean(true);
pub const FALSE: Value = Value::Boolean(false);

pub type BuiltinFn = fn(args: Vec<Rc<Value>>, pos: Position) -> Result<Rc<Value>, EvalError>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Function(Function),
    String(String),
    Array(Vec<Rc<Value>>),

    Builtin(BuiltinFn),

    Returned(Rc<Value>),

    VOID,
}

impl Value {
    pub fn new_builtin(value: BuiltinFn) -> Rc<Self> {
        Rc::new(Value::Builtin(value))
    }
    pub fn new_string(value: String) -> Rc<Self> {
        Rc::new(Value::String(value))
    }
    pub fn new_integer(value: i64) -> Rc<Self> {
        Rc::new(Value::Integer(value))
    }
    pub fn new_float(value: f64) -> Rc<Self> {
        Rc::new(Value::Float(value))
    }
    pub fn new_returned(value: Rc<Value>) -> Rc<Self> {
        Rc::new(Value::Returned(value))
    }
    pub fn new_function(
        params: Vec<String>,
        body: Rc<Block>,
        env: Rc<RefCell<ExecutionEnvironment>>,
    ) -> Rc<Self> {
        Rc::new(Value::Function(Function::new(params, body, env)))
    }
    pub fn new_array(els: Vec<Rc<Value>>) -> Rc<Self> {
        Rc::new(Value::Array(els))
    }

    pub fn from_native_bool(value: bool) -> Rc<Self> {
        if value {
            TRUE.rc()
        } else {
            FALSE.rc()
        }
    }

    // Returns the inner value of a Returned variant or VOID if not a Returned variant
    pub fn unwrap_returned(&self) -> Rc<Value> {
        match self {
            Value::Returned(value) => Rc::clone(&value),
            _ => VOID.rc(),
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Value::VOID => true,
            _ => false,
        }
    }

    pub fn is_returned(&self) -> bool {
        match self {
            Value::Returned(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn rc(self) -> Rc<Self> {
        Rc::new(self)
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::VOID => "void".to_string(),
            Value::Integer(value) => value.to_string(),
            Value::Float(value) => value.to_string(),
            Value::Boolean(value) => value.to_string(),
            Value::Returned(value) => value.to_string(),
            Value::String(value) => value.to_string(),
            Value::Function(func) => {
                format!("fn({})", func.params.join(", "))
            }
            Value::Builtin(_) => "builtin".to_string(),
            Value::Array(array) => {
                format!(
                    "[{}]",
                    array
                        .iter()
                        .map(|el| el.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}
