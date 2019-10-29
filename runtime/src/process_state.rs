use crate::{Machine, Value};
use bytecode::{FunctionID, LabelIndex, Program};
use slog::Logger;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProcessState {
    stack: Vec<StackFrame>,
}

impl ProcessState {
    pub fn empty() -> Self { ProcessState::default() }

    pub fn at_function_entry(function: FunctionID) -> Self {
        ProcessState {
            stack: vec![StackFrame {
                function,
                ip: 0,
                locals: Vec::new(),
            }],
        }
    }

    pub fn stack(&self) -> &[StackFrame] { &self.stack }

    pub fn step(&mut self, program: &Program, ctx: Ctx<'_>) -> Result<(), Fault> {
        unimplemented!()
    }
}

/// Extra context passed to the [`ProcessState`] when stepping.
pub struct Ctx<'a> {
    logger: &'a Logger,
    machine: &'a dyn Machine,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackFrame {
    /// The function currently being executed.
    function: FunctionID,
    /// The [`Instruction`] index within the current [`Function`]'s body.
    ip: usize,
    locals: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum Fault {
    #[error("No such label with index {0}")]
    InvalidLabel(LabelIndex),
}
