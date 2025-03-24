use std::sync::atomic::Ordering;

use crate::DEBUG_DROP;

use super::{Object, ObjectImpl};

#[derive(Debug, PartialEq)]
pub struct FloatObject(pub f64);

impl FloatObject {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn new_object(value: f64) -> Object {
        Object::FLOAT(FloatObject::new(value))
    }

    pub fn negated_object(&self) -> Object {
        Object::FLOAT(FloatObject::new(-self.0))
    }
}

impl ObjectImpl for FloatObject {
    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}

impl Drop for FloatObject {
    fn drop(&mut self) {
        if DEBUG_DROP.load(Ordering::SeqCst) {
            println!("{self:?} dropped!");
        }
    }
}
