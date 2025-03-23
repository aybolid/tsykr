mod boolean;
mod float;
mod function;
mod integer;

pub use boolean::*;
pub use float::*;
pub use function::*;
pub use integer::*;

#[derive(Debug, PartialEq)]
pub enum Object {
    INTEGER(IntegerObject),
    FLOAT(FloatObject),
    BOOLEAN(BooleanObject),
    FUNCTION(FunctionObject),
}

impl Object {
    #[allow(unused)]
    pub fn inspect(&self) -> String {
        match self {
            Object::INTEGER(i) => i.inspect(),
            Object::FLOAT(f) => f.inspect(),
            Object::BOOLEAN(b) => b.inspect(),
            Object::FUNCTION(f) => f.inspect(),
        }
    }
}

pub trait ObjectImpl {
    #[allow(unused)]
    fn inspect(&self) -> String;
}
