use std::{collections::HashMap, sync::Arc};

use super::Object;

#[derive(Debug, Clone)]
pub struct ExecEnvironment {
    store: HashMap<String, Arc<Object>>,
}

impl ExecEnvironment {
    pub fn new() -> Self {
        let store = HashMap::new();
        Self { store }
    }

    pub fn get(&self, key: &str) -> Option<Arc<Object>> {
        self.store.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: Arc<Object>) {
        self.store.insert(key, value);
    }
}
