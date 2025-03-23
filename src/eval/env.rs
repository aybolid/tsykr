use super::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

#[derive(Debug, Clone)]
pub struct ExecEnvironment {
    store: Rc<RefCell<HashMap<String, Arc<Object>>>>,
    outer: Option<Rc<ExecEnvironment>>, // Reference to outer/parent environment
}

impl ExecEnvironment {
    pub fn new() -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: None,
        }
    }

    // Create a new environment with access to outer environment
    pub fn new_enclosed(outer: Rc<ExecEnvironment>) -> Self {
        Self {
            store: Rc::new(RefCell::new(HashMap::new())),
            outer: Some(outer),
        }
    }

    pub fn get(&self, key: &str) -> Option<Arc<Object>> {
        // First check in current environment
        if let Some(obj) = self.store.borrow().get(key) {
            return Some(Arc::clone(obj));
        }

        // If not found and we have an outer environment, check there
        if let Some(outer) = &self.outer {
            return outer.get(key);
        }

        None
    }

    pub fn set(&self, key: String, value: Arc<Object>) {
        self.store.borrow_mut().insert(key, value);
    }

    // Optional: method to directly set in current scope without checking outer
    pub fn set_local(&self, key: String, value: Arc<Object>) {
        self.store.borrow_mut().insert(key, value);
    }
}

impl PartialEq for ExecEnvironment {
    fn eq(&self, other: &Self) -> bool {
        // For environments, we can consider them equal if they point to the same store
        Rc::ptr_eq(&self.store, &other.store)
    }
}
