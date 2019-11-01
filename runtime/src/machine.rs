#[allow(unused_imports)]
use crate::Process;
use crate::Value;
use std::{collections::HashMap, sync::Mutex};

/// The interface a [`Process`] can use to interact with the outside world
/// and other processes.
pub trait Machine {
    fn load_global(&self, name: &str) -> Option<Value>;
    fn store_global(&self, name: &str, value: Value);
    fn call(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, CallFailed>;
}

#[derive(Default)]
pub struct BasicMachine {
    pub globals: Mutex<HashMap<String, Value>>,
    pub functions: Mutex<
        HashMap<
            String,
            Box<dyn Fn(&[Value]) -> Result<Option<Value>, CallFailed>>,
        >,
    >,
}

impl BasicMachine {
    pub fn new() -> Self { BasicMachine::default() }

    pub fn register_function<F>(&self, name: &str, function: F)
    where
        F: Fn(&[Value]) -> Result<Option<Value>, CallFailed> + 'static,
    {
        let mut functions = self.functions.lock().unwrap();
        functions.insert(name.to_string(), Box::new(function));
    }
}

impl Machine for BasicMachine {
    fn load_global(&self, name: &str) -> Option<Value> {
        self.globals.lock().unwrap().get(name).cloned()
    }

    fn store_global(&self, name: &str, value: Value) {
        self.globals.lock().unwrap().insert(name.to_string(), value);
    }

    fn call(
        &self,
        name: &str,
        args: &[Value],
    ) -> Result<Option<Value>, CallFailed> {
        let functions = self.functions.lock().unwrap();
        let function =
            functions.get(name).ok_or(CallFailed::UnknownFunction)?;

        (function)(args)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, thiserror::Error)]
pub enum CallFailed {
    #[error("Unknown function")]
    UnknownFunction,
    #[error("{0}")]
    Custom(&'static str),
}
