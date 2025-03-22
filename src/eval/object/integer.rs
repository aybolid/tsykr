use super::{Object, ObjectKind};

pub struct Integer(i64);

impl Integer {
    pub fn new(value: i64) -> Self {
        Integer(value)
    }
}

impl Object for Integer {
    fn kind(&self) -> ObjectKind {
        ObjectKind::INTEGER
    }

    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
