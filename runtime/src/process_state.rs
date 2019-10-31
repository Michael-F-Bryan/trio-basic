use crate::{Machine, Process, Value};
use bytecode::{FunctionID, Instruction, LabelIndex, Program};
#[allow(unused_imports)]
use bytecode::Function;
use slog::{Logger};
use std::ops::Try;

/// Internal [`Process`] state.
///
/// You're probably here for [`ProcessState::step()`].
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProcessState {
    stack: Vec<StackFrame>,
    values: Vec<Value>,
}

impl ProcessState {
    pub fn empty() -> Self { ProcessState::default() }

    pub fn at_function_entry(function_id: FunctionID) -> Self {
        ProcessState {
            stack: vec![StackFrame { function_id, ip: 0 }],
            values: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[StackFrame] { &self.stack }

    pub fn step(&mut self, program: &Program, ctx: Ctx<'_>) -> Continuation {
        let instruction = self.fetch(program, ctx.logger)?;

        slog::trace!(ctx.logger, "Executing an instruction";
            "instruction" => format_args!("{:?}", instruction),
            self.stack.last().expect("No stack frames found!"));

        self.execute(instruction, ctx)
    }

    fn execute(
        &mut self,
        instruction: Instruction,
        ctx: Ctx<'_>,
    ) -> Continuation {
        match instruction {
            Instruction::Return => self.on_return(),
            Instruction::CallUserFunction { function_id } => 
            {
                self.stack.push(StackFrame { function_id, ip: 0 });
                // TODO: Do we want to check for infinite recursion?
                Continuation::Continue
            }

            _ => unimplemented!("We can't yet handle {:#?}", instruction),
        }
    }

    fn on_return(&mut self) -> Continuation {
        if self.stack.is_empty() {
            // returned when haven't got a stack frame
            return Continuation::Fault(Fault::EmptyStack);
        }

        self.stack.pop();

        if self.stack.is_empty() {
            // returned from the top-level function. We're done.
            Continuation::Halt
        } else {
            Continuation::Continue
        }
    }

    fn fetch(
        &self,
        program: &Program,
        logger: &Logger,
    ) -> Result<Instruction, Fault> {
        let StackFrame {
            function_id,
            ip,
            ..
        } = self.stack.last().ok_or(Fault::EmptyStack)?;

        let function = program
            .functions
            .get(*function_id)
            .ok_or_else(|| Fault::UnknownFunction(*function_id))?;

        let instruction = function.body.get(*ip).copied().ok_or_else(|| {
            Fault::InstructionOutOfBounds {
                function: *function_id,
                instruction: *ip,
            }
        })?;

        slog::trace!(logger, "Decoded an instruction";
            "function" => function_id,
            "instruction-pointer" => ip,
            "instruction" => format_args!("{:?}", instruction));

        Ok(instruction)
    }
}

/// Extra context passed to the [`ProcessState`] when stepping.
pub struct Ctx<'a> {
    logger: &'a Logger,
    machine: &'a dyn Machine,
}

impl<'a> Ctx<'a> {
    pub(crate) fn new(logger: &'a Logger, machine: &'a dyn Machine) -> Self {
        Ctx { logger, machine }
    }

    pub fn for_process(process: &'a Process) -> Self {
        Ctx::new(&process.logger, &*process.machine)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StackFrame {
    /// The function currently being executed.
    pub function_id: FunctionID,
    /// The [`Instruction`] index within the current [`Function`]'s body.
    pub ip: usize,
}

impl slog::KV for StackFrame {
    fn serialize(&self, _record: &slog::Record, ser: &mut dyn slog::Serializer) -> slog::Result {
        ser.emit_usize("function-id", self.function_id)?;
        ser.emit_usize("ip", self.ip)?;
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NopMachine;
    use slog::{Discard, Logger};
    use std::sync::Arc;

    fn setup() -> (Logger, Arc<dyn Machine>, ProcessState) {
        (
            Logger::root(Discard, slog::o!()),
            Arc::new(NopMachine),
            ProcessState::default(),
        )
    }

    #[test]
    fn return_from_last_stack_frame() {
        let (logger, machine, mut state) = setup();
        let ctx = Ctx::new(&logger, &*machine);
        state.stack.push(StackFrame { function_id: 0, ip: 0 });

        let cont = state.execute(Instruction::Return, ctx);

        assert_eq!(cont, Continuation::Halt);
        assert!(state.stack.is_empty());
    }

    #[test]
    fn return_from_penultimate_stack_frame() {
        let (logger, machine, mut state) = setup();
        let ctx = Ctx::new(&logger, &*machine);
        state.stack.push(StackFrame { function_id: 0, ip: 0 });
        state.stack.push(StackFrame { function_id: 0, ip: 0 });

        let cont = state.execute(Instruction::Return, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.stack.len(), 1);
    }

    #[test]
    fn call_user_function_no_args() {
        let (logger, machine, mut state) = setup();
        let ctx = Ctx::new(&logger, &*machine);
        state.stack.push(StackFrame { function_id: 0, ip: 0 });
        let instr = Instruction::CallUserFunction { function_id: 42 };
        
        let cont = state.execute(instr, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.stack.len(), 2);
        assert_eq!(state.stack.last().unwrap(), &StackFrame { function_id: 42, ip: 0 });
    }
}
