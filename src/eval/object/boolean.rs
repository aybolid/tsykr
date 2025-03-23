use super::{Object, ObjectImpl};

pub const TRUE: Object = Object::BOOLEAN(BooleanObject(true));
pub const FALSE: Object = Object::BOOLEAN(BooleanObject(false));

/// BooleanObject represents a boolean value in the language.
/// Use `TRUE` or `FALSE` as there is no reason to allocate a new instances.
#[derive(Debug, PartialEq)]
pub struct BooleanObject(pub bool);

impl ObjectImpl for BooleanObject {
    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
