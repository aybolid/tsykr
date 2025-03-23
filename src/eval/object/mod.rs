mod boolean;
mod float;
mod integer;

pub use boolean::*;
pub use float::*;
pub use integer::*;

#[derive(Debug, PartialEq)]
pub enum Object {
    INTEGER(IntegerObject),
    FLOAT(FloatObject),
    BOOLEAN(BooleanObject),
}

impl Object {
    #[allow(unused)]
    pub fn inspect(&self) -> String {
        match self {
            Object::INTEGER(i) => i.inspect(),
            Object::FLOAT(f) => f.inspect(),
            Object::BOOLEAN(b) => b.inspect(),
        }
    }
}

pub trait ObjectImpl {
    #[allow(unused)]
    fn inspect(&self) -> String;
}
