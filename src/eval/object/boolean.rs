use std::sync::atomic::Ordering;

use crate::DEBUG_DROP;

use super::{Object, ObjectImpl};

pub const TRUE: Object = Object::BOOLEAN(BooleanObject(true));
pub const FALSE: Object = Object::BOOLEAN(BooleanObject(false));

/// BooleanObject represents a boolean value in the language.
/// Use `TRUE` or `FALSE` as there is no reason to allocate a new instances.
#[derive(Debug, PartialEq)]
pub struct BooleanObject(pub bool);

impl BooleanObject {
    pub fn negated_object(&self) -> Object {
        if self.0 {
            FALSE
        } else {
            TRUE
        }
    }

    pub fn object_from_bool(value: bool) -> Object {
        if value {
            TRUE
        } else {
            FALSE
        }
    }
}

impl ObjectImpl for BooleanObject {
    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}

impl Drop for BooleanObject {
    fn drop(&mut self) {
        if DEBUG_DROP.load(Ordering::SeqCst) {
            println!("{self:?} dropped!");
        }
    }
}
