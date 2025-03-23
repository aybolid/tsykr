use super::{Object, ObjectImpl};
use crate::{
    eval::ExecEnvironment,
    parser::{Block, Identifier},
};
use std::rc::Rc;

#[derive(Debug)]
pub struct FunctionObject {
    pub env: Rc<ExecEnvironment>,
    pub params: Vec<Identifier>,
    pub body: Block,
}

impl FunctionObject {
    pub fn new(env: Rc<ExecEnvironment>, params: Vec<Identifier>, body: Block) -> Self {
        Self { env, params, body }
    }

    pub fn new_object(env: Rc<ExecEnvironment>, params: Vec<Identifier>, body: Block) -> Object {
        Object::FUNCTION(Self::new(env, params, body))
    }
}

impl PartialEq for FunctionObject {
    fn eq(&self, other: &Self) -> bool {
        // Compare parameters and body
        self.params == other.params &&
        self.body == other.body &&
        // For environments, we can either check if they're the same reference
        Rc::ptr_eq(&self.env, &other.env)
        // Or if you need deeper equality, implement a custom comparison
        // *self.env == *other.env
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
