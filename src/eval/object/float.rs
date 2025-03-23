use super::{Object, ObjectImpl};

#[derive(Debug, PartialEq)]
pub struct FloatObject(f64);

impl FloatObject {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn new_object(value: f64) -> Object {
        Object::FLOAT(FloatObject::new(value))
    }
}

impl ObjectImpl for FloatObject {
    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
