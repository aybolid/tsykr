mod integer;

pub use integer::*;

#[derive(Debug, PartialEq)]
pub enum ObjectKind {
    INTEGER,
}

pub trait Object {
    fn kind(&self) -> ObjectKind;
    fn inspect(&self) -> String;
}
