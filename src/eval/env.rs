use std::collections::HashMap;

use super::Object;

pub struct ExecEnvironment {
    store: HashMap<String, Box<dyn Object>>,
}

impl ExecEnvironment {
    pub fn new() -> Self {
        let store = HashMap::new();
        Self { store }
    }

    pub fn get(&self, key: &str) -> Option<&Box<dyn Object>> {
        self.store.get(key)
    }

    pub fn set(&mut self, key: String, value: Box<dyn Object>) {
        self.store.insert(key, value);
    }
}
