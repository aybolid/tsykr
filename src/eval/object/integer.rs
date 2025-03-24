use std::sync::atomic::Ordering;

use crate::DEBUG_DROP;

use super::{Object, ObjectImpl};

#[derive(Debug, PartialEq)]
pub struct IntegerObject(pub i64);

impl IntegerObject {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn new_object(value: i64) -> Object {
        Object::INTEGER(Self::new(value))
    }

    pub fn negated_object(&self) -> Object {
        Object::INTEGER(Self::new(-self.0))
    }
}

impl ObjectImpl for IntegerObject {
    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}

impl Drop for IntegerObject {
    fn drop(&mut self) {
        if DEBUG_DROP.load(Ordering::SeqCst) {
            println!("{self:?} dropped!");
        }
    }
}
