use super::{Object, ObjectImpl};
use crate::{
    eval::{Eval, EvalError, ExecEnvironment},
    parser::{Block, Identifier},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct FunctionObject {
    pub env: Rc<RefCell<ExecEnvironment>>,
    pub params: Vec<Identifier>,
    pub body: Block,
}

impl FunctionObject {
    pub fn new(env: Rc<RefCell<ExecEnvironment>>, params: Vec<Identifier>, body: Block) -> Self {
        Self { env, params, body }
    }

    pub fn new_object(
        env: Rc<RefCell<ExecEnvironment>>,
        params: Vec<Identifier>,
        body: Block,
    ) -> Object {
        Object::FUNCTION(Self::new(env, params, body))
    }

    pub fn call(&self, args: Vec<Rc<Object>>) -> Result<Option<Rc<Object>>, EvalError> {
        let mut function_env = ExecEnvironment::new_enclosed(Rc::clone(&self.env));

        for (param, arg) in self.params.iter().zip(args.iter()) {
            function_env.set(param.to_string(), Rc::clone(arg));
        }

        self.body.eval(Rc::new(RefCell::new(function_env)))
    }
}

impl PartialEq for FunctionObject {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params && self.body == other.body && Rc::ptr_eq(&self.env, &other.env)
    }
}

impl ObjectImpl for FunctionObject {
    fn inspect(&self) -> String {
        let mut out = String::from("fn(");
        out.push_str(
            &self
                .params
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        );
        out.push(')');
        out
    }
}
