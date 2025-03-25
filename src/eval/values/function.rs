use std::{cell::RefCell, rc::Rc};

use crate::{eval::ExecutionEnvironment, parser::Block};

#[derive(Debug, PartialEq)]
pub struct Function {
    pub captured_env: Rc<RefCell<ExecutionEnvironment>>,
    pub params: Vec<String>,
    pub body: Rc<Block>,
}

impl Function {
    pub fn new(
        params: Vec<String>,
        body: Rc<Block>,
        env: Rc<RefCell<ExecutionEnvironment>>,
    ) -> Self {
        Function {
            captured_env: env,
            params,
            body,
        }
    }
}
