use super::ObjectImpl;

#[derive(Debug, PartialEq)]
pub struct IntegerObject(pub i64);

impl IntegerObject {
    pub fn new(value: i64) -> Self {
        Self(value)
    }
}

impl ObjectImpl for IntegerObject {
    fn inspect(&self) -> String {
        format!("{}", self.0)
    }
}
