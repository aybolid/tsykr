use super::{Object, ObjectKind};

pub const TRUE: BooleanObject = BooleanObject(true);
pub const FALSE: BooleanObject = BooleanObject(false);

/// BooleanObject represents a boolean value in the language.
/// Use `TRUE` or `FALSE` as there is no reason to allocate a new instances.
pub struct BooleanObject(bool);

impl Object for BooleanObject {
    fn kind(&self) -> ObjectKind {
        ObjectKind::BOOLEAN
    }

    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
