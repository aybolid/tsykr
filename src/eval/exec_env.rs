use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Value;

pub enum ExecutionEnvironment {
    Global(GlobalEnvironment),
    /// Scoped environment for local variables. Includes parent execution environment.
    Local(LocalEnvironment),
}

impl ExecutionEnvironment {
    pub fn new_global() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ExecutionEnvironment::Global(
            GlobalEnvironment::new(),
        )))
    }

    pub fn new_local(parent: Rc<RefCell<ExecutionEnvironment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ExecutionEnvironment::Local(
            LocalEnvironment::new(parent),
        )))
    }

    pub fn debug_string(&self) -> String {
        match self {
            ExecutionEnvironment::Global(env) => format!("{:#?}", env.store),
            ExecutionEnvironment::Local(env) => format!("{:#?}", env.store),
        }
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

pub struct GlobalEnvironment {
    store: HashMap<String, Rc<Value>>,
}

impl GlobalEnvironment {
    pub fn new() -> Self {
        GlobalEnvironment {
            store: HashMap::new(),
        }
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
