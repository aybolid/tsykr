use super::ObjectImpl;

#[derive(Debug, PartialEq)]
pub struct FloatObject(f64);

impl FloatObject {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl ObjectImpl for FloatObject {
    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
