use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::{builtins, Value};

#[derive(Debug, PartialEq)]
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

    pub fn assign(&mut self, name: &str, value: Rc<Value>) -> bool {
        match self {
            ExecutionEnvironment::Global(env) => {
                if env.store.contains_key(name) {
                    env.store.insert(name.to_string(), value);
                    true
                } else {
                    false
                }
            }
            ExecutionEnvironment::Local(env) => {
                if env.store.contains_key(name) {
                    env.store.insert(name.to_string(), value);
                    return true;
                }

                fn find_and_assign(
                    current_env: &mut Rc<RefCell<ExecutionEnvironment>>,
                    name: &str,
                    value: Rc<Value>,
                ) -> bool {
                    match &mut *current_env.borrow_mut() {
                        ExecutionEnvironment::Global(global_env) => {
                            if global_env.store.contains_key(name) {
                                global_env.store.insert(name.to_string(), value);
                                true
                            } else {
                                false
                            }
                        }
                        ExecutionEnvironment::Local(local_env) => {
                            if local_env.store.contains_key(name) {
                                local_env.store.insert(name.to_string(), value);
                                return true;
                            }

                            find_and_assign(&mut local_env.parent, name, value)
                        }
                    }
                }

                find_and_assign(&mut env.parent, name, value)
            }
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

#[derive(Debug, PartialEq)]
pub struct GlobalEnvironment {
    store: HashMap<String, Rc<Value>>,
}

impl GlobalEnvironment {
    pub fn new() -> Self {
        let mut env = GlobalEnvironment {
            store: HashMap::new(),
        };
        builtins::register_builtins(&mut env);
        env
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
    pub parent: Rc<RefCell<ExecutionEnvironment>>,
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
