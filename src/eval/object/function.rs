use super::{Object, ObjectImpl};
use crate::{
    eval::{Eval, EvalError, ExecEnvironment},
    parser::{Block, Identifier},
    DEBUG_DROP,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::atomic::Ordering,
};

#[derive(Debug)]
pub struct FunctionObject {
    pub env: Weak<RefCell<ExecEnvironment>>,
    pub params: Vec<Identifier>,
    pub body: Block,
}

impl FunctionObject {
    pub fn new(env: Weak<RefCell<ExecEnvironment>>, params: Vec<Identifier>, body: Block) -> Self {
        Self { env, params, body }
    }

    pub fn new_object(
        env: Weak<RefCell<ExecEnvironment>>,
        params: Vec<Identifier>,
        body: Block,
    ) -> Object {
        Object::FUNCTION(Self::new(env, params, body))
    }

    pub fn call(&self, args: Vec<Rc<Object>>) -> Result<Option<Rc<Object>>, EvalError> {
        let self_env = self.env.upgrade().unwrap();
        let mut function_env = ExecEnvironment::new_enclosed(Rc::clone(&self_env));

        for (param, arg) in self.params.iter().zip(args.iter()) {
            function_env.set(param.to_string(), Rc::clone(arg));
        }

        self.body.eval(Rc::new(RefCell::new(function_env)))
    }
}

impl PartialEq for FunctionObject {
    fn eq(&self, other: &Self) -> bool {
        let self_env = self.env.upgrade().unwrap();
        let other_env = other.env.upgrade().unwrap();
        self.params == other.params && self.body == other.body && Rc::ptr_eq(&self_env, &other_env)
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

impl Drop for FunctionObject {
    fn drop(&mut self) {
        if DEBUG_DROP.load(Ordering::SeqCst) {
            println!("{} dropped!", self.inspect());
        }
    }
}
