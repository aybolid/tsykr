use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{eval::ExecutionEnvironment, parser::Block};

#[derive(PartialEq)]
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

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function")
            .field("captured_env", &self.captured_env.as_ptr())
            .field("params", &self.params)
            .field("body", &Rc::as_ptr(&self.body))
            .finish()
    }
}
