use super::{Object, ObjectKind};

pub struct FloatObject(f64);

impl FloatObject {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl Object for FloatObject {
    fn kind(&self) -> ObjectKind {
        ObjectKind::FLOAT
    }

    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
