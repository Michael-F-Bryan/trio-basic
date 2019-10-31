use crate::{Machine, Process, Value};
#[allow(unused_imports)]
use bytecode::Function;
use bytecode::{FunctionID, Instruction, LabelIndex, Program, StringIndex};
use slog::Logger;
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
            self.stack_frame()?);

        self.execute(instruction, &program.string_table, ctx)
    }

    fn execute(
        &mut self,
        instruction: Instruction,
        strings: &[String],
        ctx: Ctx<'_>,
    ) -> Continuation {
        match instruction {
            Instruction::Return => return self.on_return(),
            Instruction::CallUserFunction { function_id } => {
                return self.on_call_user_func(function_id, ctx)
            },
            Instruction::PushInteger(i) => self.values.push(Value::Integer(i)),
            Instruction::PushDouble(d) => self.values.push(Value::Double(d)),
            Instruction::PushBoolean(b) => self.values.push(Value::Boolean(b)),
            Instruction::PushString { string_id } => {
                let string = strings
                    .get(string_id)
                    .ok_or_else(|| Fault::UnknownString { string_id })?;
                self.values.push(Value::String(string.clone()));
            },
            Instruction::PushGlobal { variable_id } => {
                self.on_push_global(variable_id, strings, ctx)?
            },

            _ => unimplemented!("We can't yet handle {:?}", instruction),
        }

        self.advance_ip();
        Continuation::Continue
    }

    fn on_push_global(
        &mut self,
        string_id: StringIndex,
        strings: &[String],
        ctx: Ctx<'_>,
    ) -> Continuation {
        let variable_name = strings
            .get(string_id)
            .ok_or_else(|| Fault::UnknownString { string_id })?;

        let value = ctx
            .machine
            .load_global(variable_name)
            .ok_or_else(|| Fault::UnknownGlobal(variable_name.to_string()))?;
        self.values.push(value);

        Continuation::Continue
    }

    fn advance_ip(&mut self) {
        let mut sf = self.stack_frame_mut().expect(
            "Trying to increment the instruction pointer without a stack frame",
        );
        sf.ip += 1;
    }

    fn on_call_user_func(
        &mut self,
        function_id: FunctionID,
        ctx: Ctx<'_>,
    ) -> Continuation {
        self.stack.push(StackFrame { function_id, ip: 0 });
        slog::trace!(ctx.logger, "Calling a user function";
                    "function-id" => function_id,
                    "new-stack-depth" => self.stack.len());

        // TODO: Do we want to check for infinite recursion?
        Continuation::Continue
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

    fn stack_frame(&self) -> Result<StackFrame, Fault> {
        self.stack.last().copied().ok_or(Fault::EmptyStack)
    }

    fn stack_frame_mut(&mut self) -> Result<&mut StackFrame, Fault> {
        self.stack.last_mut().ok_or(Fault::EmptyStack)
    }

    fn fetch(
        &self,
        program: &Program,
        logger: &Logger,
    ) -> Result<Instruction, Fault> {
        let stack_frame = self.stack_frame()?;
        let StackFrame { function_id, ip } = stack_frame;

        let function = program
            .functions
            .get(function_id)
            .ok_or_else(|| Fault::UnknownFunction(function_id))?;

        let instruction = function.body.get(ip).copied().ok_or_else(|| {
            Fault::InstructionOutOfBounds {
                function: function_id,
                instruction: ip,
            }
        })?;

        slog::trace!(logger, "Decoded an instruction";
            "instruction" => format_args!("{:?}", instruction),
            stack_frame);

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
    fn serialize(
        &self,
        _record: &slog::Record,
        ser: &mut dyn slog::Serializer,
    ) -> slog::Result {
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
    #[error("No such string with ID {string_id}")]
    UnknownString { string_id: StringIndex },
    #[error("No global called \"{0}\"")]
    UnknownGlobal(String),
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
    use crate::BasicMachine;
    use slog::{Discard, Logger};
    use std::sync::Arc;

    macro_rules! ps_test {
        ($name:ident, |$machine:ident, $ctx:ident, $strings:ident, mut $state:ident| $body:block) => {
            #[test]
            fn $name() {
                let logger = Logger::root(Discard, slog::o!());
                let $machine = Arc::new(BasicMachine::default());
                let mut $state = ProcessState::default();
                $state.stack.push(StackFrame {
                    function_id: 0,
                    ip: 0,
                });

                let $ctx = Ctx::new(&logger, &*$machine);

                let $strings = vec![
                    String::from("some string"),
                    String::from("global"),
                    String::from("some_function"),
                ];

                $body
            }
        };
    }

    ps_test!(return_from_last_stack_frame, |_mac, ctx, _s, mut state| {
        let cont = state.execute(Instruction::Return, &[], ctx);

        assert_eq!(cont, Continuation::Halt);
        assert!(state.stack.is_empty());
    });

    ps_test!(
        return_from_penultimate_stack_frame,
        |_mac, ctx, _s, mut state| {
            state.stack.push(StackFrame {
                function_id: 0,
                ip: 0,
            });

            let cont = state.execute(Instruction::Return, &[], ctx);

            assert_eq!(cont, Continuation::Continue);
            assert_eq!(state.stack.len(), 1);
        }
    );

    ps_test!(call_user_function_no_args, |_mac, ctx, _s, mut state| {
        let instr = Instruction::CallUserFunction { function_id: 42 };

        let cont = state.execute(instr, &[], ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.stack.len(), 2);
        assert_eq!(
            state.stack.last().unwrap(),
            &StackFrame {
                function_id: 42,
                ip: 0
            }
        );
    });

    ps_test!(push_int, |_mac, ctx, _s, mut state| {
        let instr = Instruction::PushInteger(42);

        let cont = state.execute(instr, &[], ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Integer(42));
    });

    ps_test!(push_double, |_mac, ctx, _s, mut state| {
        let instr = Instruction::PushDouble(3.14);

        let cont = state.execute(instr, &[], ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Double(3.14));
    });

    ps_test!(push_bool, |_mac, ctx, _s, mut state| {
        let instr = Instruction::PushBoolean(true);

        let cont = state.execute(instr, &[], ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Boolean(true));
    });

    ps_test!(push_string_from_string_pool, |_mac, ctx, s, mut state| {
        let instr = Instruction::PushString { string_id: 0 };

        let cont = state.execute(instr, &s, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::String(String::from("some string")));
    });

    ps_test!(push_unknown_string, |_mac, ctx, s, mut state| {
        let instr = Instruction::PushString { string_id: 42 };

        let cont = state.execute(instr, &s, ctx);

        assert_eq!(
            cont,
            Continuation::Fault(Fault::UnknownString { string_id: 42 })
        );
    });

    ps_test!(push_global_variable, |mac, ctx, s, mut state| {
        mac.store_global(&s[1], Value::Integer(42));
        let instr = Instruction::PushGlobal { variable_id: 1 };

        let cont = state.execute(instr, &s, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Integer(42));
    });
}
