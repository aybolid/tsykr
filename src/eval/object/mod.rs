mod float;
mod integer;

pub use float::*;
pub use integer::*;

#[derive(Debug, PartialEq)]
pub enum ObjectKind {
    INTEGER,
    FLOAT,
}

pub trait Object {
    #[allow(unused)]
    fn kind(&self) -> ObjectKind;
    fn inspect(&self) -> String;
}
