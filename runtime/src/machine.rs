#[allow(unused_imports)]
use crate::Process;
use crate::Value;
use std::{collections::HashMap, sync::Mutex};

/// The interface a [`Process`] can use to interact with the outside world
/// and other processes.
pub trait Machine {
    fn load_global(&self, name: &str) -> Option<Value>;
    fn store_global(&self, name: &str, value: Value);
}

#[derive(Debug, Default)]
pub struct BasicMachine {
    pub globals: Mutex<HashMap<String, Value>>,
}

impl Machine for BasicMachine {
    fn load_global(&self, name: &str) -> Option<Value> {
        self.globals.lock().unwrap().get(name).cloned()
    }

    fn store_global(&self, name: &str, value: Value) {
        self.globals.lock().unwrap().insert(name.to_string(), value);
    }
}
