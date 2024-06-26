use rinonguci_script::new_error;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Object;

#[derive(Debug, PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            store: HashMap::new(),
            outer: None,
        }))
    }

    pub fn new_enclosed_environment(outer: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: String) -> Option<Object> {
        match self.store.get(&name) {
            Some(val) => Some(val.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn init(&mut self, name: String, val: Object) -> Object {
        self.store.insert(name, val.clone());
        val
    }

    pub fn assign(&mut self, name: String, val: Object) -> Object {
        match self.store.get(&name) {
            Some(_) => {
                self.store.insert(name, val.clone());
                val
            }
            None => match &self.outer {
                Some(outer) => outer.borrow_mut().assign(name, val),
                None => new_error!("unknown operator"),
            },
        }
    }
}
