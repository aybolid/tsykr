use super::{Object, ObjectKind};

pub struct IntegerObject(i64);

impl IntegerObject {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

impl Object for IntegerObject {
    fn kind(&self) -> ObjectKind {
        ObjectKind::INTEGER
    }

    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
