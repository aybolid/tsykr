use std::rc::Rc;

pub const VOID: Value = Value::VOID;
pub const TRUE: Value = Value::Boolean(true);
pub const FALSE: Value = Value::Boolean(false);

pub enum Value {
    VOID,
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl Value {
    pub fn new_integer(value: i64) -> Rc<Self> {
        Rc::new(Value::Integer(value))
    }
    pub fn new_float(value: f64) -> Rc<Self> {
        Rc::new(Value::Float(value))
    }

    pub fn rc(self) -> Rc<Self> {
        Rc::new(self)
    }
}
