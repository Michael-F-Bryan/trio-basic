use crate::{Machine, Process, Value};
use bytecode::{FunctionID, LabelIndex, Program, Instruction, Function};
use slog::Logger;
use std::ops::Try;

/// Internal [`Process`] state.
///
/// You're probably here for [`ProcessState::step()`].
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

    pub fn step(
        &mut self,
        program: &Program,
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        let StackFrame { function: function_id, ip, .. } =
            self.stack.last().ok_or(Fault::EmptyStack)?;

        let function = program
            .functions
            .get(*function_id)
            .ok_or_else(|| Fault::UnknownFunction(*function_id))?;

        let instruction = function.body.get(*ip).ok_or_else(|| {
            Fault::InstructionOutOfBounds {
                function: *function_id,
                instruction: *ip,
            }
        })?;

        match instruction {
            Instruction::Return => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

/// Extra context passed to the [`ProcessState`] when stepping.
pub struct Ctx<'a> {
    logger: &'a Logger,
    machine: &'a dyn Machine,
}

impl<'a> Ctx<'a> {
    pub fn for_process(process: &'a Process) -> Self {
        Ctx {
            logger: &process.logger,
            machine: &*process.machine,
        }
    }
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
    #[error("Trying to execute an instruction when there are no stack frames")]
    EmptyStack,
    #[error("No such function with ID {0}")]
    UnknownFunction(FunctionID),
    #[error("Instruction out of bounds for function {function}")]
    InstructionOutOfBounds {
        function: FunctionID,
        instruction: usize,
    },
}

/// How to continue after a call to [`ProcessState::step()`].
#[derive(Debug, Clone, PartialEq)]
pub enum Continuation {
    Continue,
    Halt,
    Fault(Fault),
}

impl Try for Continuation {
    type Error = Fault;
    type Ok = ();

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Continuation::Continue | Continuation::Halt => Ok(()),
            Continuation::Fault(fault) => Err(fault),
        }
    }

    #[cold]
    fn from_error(fault: Self::Error) -> Self { Continuation::Fault(fault) }

    fn from_ok(_: Self::Ok) -> Self { Continuation::Continue }
}
