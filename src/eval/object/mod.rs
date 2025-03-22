mod boolean;
mod float;
mod integer;

pub use boolean::*;
pub use float::*;
pub use integer::*;

#[derive(Debug, PartialEq)]
pub enum ObjectKind {
    INTEGER,
    FLOAT,
    BOOLEAN,
}

pub trait Object {
    #[allow(unused)]
    fn kind(&self) -> ObjectKind;
    fn inspect(&self) -> String;
}
