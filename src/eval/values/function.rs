use std::rc::Rc;

use crate::parser::Block;

#[derive(Debug, PartialEq)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<Block>,
}

impl Function {
    pub fn new(params: Vec<String>, body: Rc<Block>) -> Self {
        Function { params, body }
    }
}
