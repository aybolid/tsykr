use std::rc::Rc;

pub const VOID: Value = Value::VOID;
pub const TRUE: Value = Value::Boolean(true);
pub const FALSE: Value = Value::Boolean(false);

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),

    Returned(Rc<Value>),

    VOID,
}

impl Value {
    pub fn new_integer(value: i64) -> Rc<Self> {
        Rc::new(Value::Integer(value))
    }
    pub fn new_float(value: f64) -> Rc<Self> {
        Rc::new(Value::Float(value))
    }
    pub fn new_returned(value: Rc<Value>) -> Rc<Self> {
        Rc::new(Value::Returned(value))
    }

    pub fn from_native_bool(value: bool) -> Rc<Self> {
        if value {
            TRUE.rc()
        } else {
            FALSE.rc()
        }
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
        }
    }
}
