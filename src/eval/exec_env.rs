use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{BuiltinFunction, PrintlnBuiltin, Value};

#[derive(Debug, PartialEq)]
pub enum ExecutionEnvironment {
    Global(GlobalEnvironment),
    /// Scoped environment for local variables. Includes parent execution environment.
    Local(LocalEnvironment),
}

impl ExecutionEnvironment {
    pub fn new_global() -> Rc<RefCell<Self>> {
        let mut env = GlobalEnvironment::new();
        env.register_builtins();
        Rc::new(RefCell::new(ExecutionEnvironment::Global(env)))
    }

    pub fn new_local(parent: Rc<RefCell<ExecutionEnvironment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ExecutionEnvironment::Local(
            LocalEnvironment::new(parent),
        )))
    }
}

impl Environment for ExecutionEnvironment {
    fn get(&self, name: &str) -> Option<Rc<Value>> {
        match self {
            ExecutionEnvironment::Global(env) => env.get(name),
            ExecutionEnvironment::Local(env) => env.get(name),
        }
    }

    fn set(&mut self, name: String, value: Rc<Value>) {
        match self {
            ExecutionEnvironment::Global(env) => env.set(name, value),
            ExecutionEnvironment::Local(env) => env.set(name, value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GlobalEnvironment {
    store: HashMap<String, Rc<Value>>,
}

impl GlobalEnvironment {
    pub fn new() -> Self {
        GlobalEnvironment {
            store: HashMap::new(),
        }
    }

    fn register_builtins(&mut self) {
        let print = PrintlnBuiltin;
        self.set(print.get_identifier(), Value::new_builtin(Box::new(print)));
    }
}

impl Environment for GlobalEnvironment {
    fn get(&self, name: &str) -> Option<Rc<Value>> {
        self.store.get(name).cloned()
    }

    fn set(&mut self, name: String, value: Rc<Value>) {
        self.store.insert(name, value);
    }
}

#[derive(Debug, PartialEq)]
pub struct LocalEnvironment {
    store: HashMap<String, Rc<Value>>,
    parent: Rc<RefCell<ExecutionEnvironment>>,
}

impl LocalEnvironment {
    pub fn new(parent: Rc<RefCell<ExecutionEnvironment>>) -> Self {
        LocalEnvironment {
            store: HashMap::new(),
            parent,
        }
    }
}

impl Environment for LocalEnvironment {
    fn get(&self, name: &str) -> Option<Rc<Value>> {
        self.store
            .get(name)
            .cloned() // clones inner rc value
            .or_else(|| self.parent.borrow().get(name))
    }

    fn set(&mut self, name: String, value: Rc<Value>) {
        self.store.insert(name, value);
    }
}

pub trait Environment {
    fn get(&self, name: &str) -> Option<Rc<Value>>;
    fn set(&mut self, name: String, value: Rc<Value>);
}
