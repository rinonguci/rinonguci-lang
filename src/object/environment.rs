use std::collections::HashMap;

use super::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Option<Object> {
        self.store.get(&name).cloned()
    }

    pub fn set(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name.clone(), val.clone());
        val
    }
}
