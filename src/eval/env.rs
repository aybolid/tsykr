use super::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct ExecEnvironment {
    pub store: HashMap<String, Rc<Object>>,
    pub outer: Option<Rc<RefCell<ExecEnvironment>>>,
}

impl ExecEnvironment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<ExecEnvironment>>) -> Self {
        let mut env = Self::new();
        env.outer = Some(outer);
        env
    }

    pub fn get(&self, key: &str) -> Option<Rc<Object>> {
        if let Some(obj) = self.store.get(key) {
            return Some(Rc::clone(obj));
        }

        if let Some(outer) = &self.outer {
            return outer.borrow().get(key);
        }

        None
    }

    pub fn set(&mut self, key: String, value: Rc<Object>) {
        self.store.insert(key, value);
    }
}
