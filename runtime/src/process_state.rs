use crate::{CallFailed, Machine, Process, Value};
#[allow(unused_imports)]
use bytecode::Function;
use bytecode::{
    FunctionID, Instruction, LabelIndex, Program, StringIndex, Type,
};
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

        self.execute(instruction, &program, ctx)
    }

    fn execute(
        &mut self,
        instruction: Instruction,
        program: &Program,
        ctx: Ctx<'_>,
    ) -> Continuation {
        match instruction {
            Instruction::Return => return self.on_return(),
            Instruction::CallUserFunction { function_id } => {
                self.on_call_user_func(function_id, ctx)?;
            },
            Instruction::PushInteger(i) => self.values.push(Value::Integer(i)),
            Instruction::PushDouble(d) => self.values.push(Value::Double(d)),
            Instruction::PushBoolean(b) => self.values.push(Value::Boolean(b)),
            Instruction::PushString { string_id } => {
                let string = program
                    .string_table
                    .get(string_id)
                    .ok_or_else(|| Fault::UnknownString { string_id })?;
                self.values.push(Value::String(string.clone()));
            },
            Instruction::LoadGlobal { variable_id } => {
                self.on_load_global(variable_id, &program.string_table, ctx)?;
            },
            Instruction::StoreGlobal { variable_id } => {
                self.on_store_global(variable_id, &program.string_table, ctx)?;
            },
            Instruction::Pop => {
                self.values
                    .pop()
                    .ok_or_else(|| Fault::PoppedFromEmptyStack)?;
            },
            Instruction::Branch {
                true_label,
                false_label,
            } => {
                self.on_branch(
                    true_label,
                    false_label,
                    self.current_function(program)?,
                )?;
                return Continuation::Continue;
            },
            Instruction::Goto { label } => {
                self.on_goto(label, self.current_function(program)?)?;
                return Continuation::Continue;
            },
            Instruction::CallBuiltinFunction { function_id, args } => {
                self.on_call_builtin(function_id, args, program, ctx)?
            },
        }

        self.advance_ip();
        Continuation::Continue
    }

    fn on_call_builtin(
        &mut self,
        function_id: StringIndex,
        arg_count: usize,
        program: &Program,
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        if self.values.len() < arg_count {
            return Err(Fault::PoppedFromEmptyStack);
        }

        let arg_start = self.values.len() - arg_count;
        let args = &self.values[arg_start..];

        let function_name =
            program.string_table.get(function_id).ok_or_else(|| {
                Fault::UnknownString {
                    string_id: function_id,
                }
            })?;

        slog::debug!(ctx.logger, "Calling a builtin function";
            "name" => function_name,
            "args" => format_args!("{:?}", args));

        let result = ctx.machine.call(function_name, args).map_err(|e| {
            Fault::CallFailed {
                function_name: function_id,
                inner: e,
            }
        })?;

        if let Some(return_value) = result {
            self.values.push(return_value);
        }

        Ok(())
    }

    fn on_goto(
        &mut self,
        label_id: LabelIndex,
        current_function: &Function,
    ) -> Result<(), Fault> {
        let mut sf = self.stack_frame_mut().expect("unreachable");
        sf.ip = current_function
            .labels
            .get(label_id)
            .copied()
            .ok_or_else(|| Fault::UnknownLabel(label_id))?;

        Ok(())
    }

    fn on_branch(
        &mut self,
        true_label: LabelIndex,
        false_label: LabelIndex,
        current_function: &Function,
    ) -> Result<(), Fault> {
        let condition = match self.values.pop() {
            Some(Value::Boolean(cond)) => cond,
            Some(other) => {
                return Err(Fault::TypeError {
                    found: other.ty(),
                    expected: Type::Boolean,
                })
            },
            None => return Err(Fault::PoppedFromEmptyStack),
        };

        let label_id = if condition { true_label } else { false_label };

        self.on_goto(label_id, current_function)
    }

    fn on_store_global(
        &mut self,
        string_id: StringIndex,
        strings: &[String],
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        let value = self
            .values
            .pop()
            .ok_or_else(|| Fault::PoppedFromEmptyStack)?;

        let variable_name = strings
            .get(string_id)
            .ok_or_else(|| Fault::UnknownString { string_id })?;

        ctx.machine.store_global(variable_name, value);

        Ok(())
    }

    fn on_load_global(
        &mut self,
        string_id: StringIndex,
        strings: &[String],
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        let variable_name = strings
            .get(string_id)
            .ok_or_else(|| Fault::UnknownString { string_id })?;

        let value = ctx
            .machine
            .load_global(variable_name)
            .ok_or_else(|| Fault::UnknownGlobal(variable_name.to_string()))?;
        self.values.push(value);

        Ok(())
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
    ) -> Result<(), Fault> {
        self.stack.push(StackFrame { function_id, ip: 0 });
        slog::trace!(ctx.logger, "Calling a user function";
                    "function-id" => function_id,
                    "new-stack-depth" => self.stack.len());

        // TODO: Do we want to check for infinite recursion?
        Ok(())
    }

    fn on_return(&mut self) -> Continuation {
        if self.stack.is_empty() {
            // returned when haven't got a stack frame
            return Continuation::Fault(Fault::EmptyCallStack);
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
        self.stack.last().copied().ok_or(Fault::EmptyCallStack)
    }

    fn stack_frame_mut(&mut self) -> Result<&mut StackFrame, Fault> {
        self.stack.last_mut().ok_or(Fault::EmptyCallStack)
    }

    fn current_function<'a>(
        &self,
        program: &'a Program,
    ) -> Result<&'a Function, Fault> {
        let StackFrame { function_id, .. } = self.stack_frame()?;

        program
            .functions
            .get(function_id)
            .ok_or_else(|| Fault::UnknownFunction(function_id))
    }

    fn fetch(
        &self,
        program: &Program,
        logger: &Logger,
    ) -> Result<Instruction, Fault> {
        let StackFrame { function_id, ip } = self.stack_frame()?;
        let function = self.current_function(program)?;

        let instruction = function.body.get(ip).copied().ok_or_else(|| {
            Fault::InstructionOutOfBounds {
                function: function_id,
                instruction: ip,
            }
        })?;

        slog::trace!(logger, "Decoded an instruction";
            "instruction" => format_args!("{:?}", instruction),
            self.stack_frame()?);

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
    UnknownLabel(LabelIndex),
    #[error("Trying to execute an instruction when there are no stack frames")]
    EmptyCallStack,
    #[error("Tried to pop a value when nothing is on the stack")]
    PoppedFromEmptyStack,
    #[error("No such function with ID {0}")]
    UnknownFunction(FunctionID),
    #[error("No such string with ID {string_id}")]
    UnknownString { string_id: StringIndex },
    #[error("Expected a {expected} but found {found}")]
    TypeError { found: Type, expected: Type },
    #[error("No global called \"{0}\"")]
    UnknownGlobal(String),
    #[error("Instruction out of bounds for function {function}")]
    InstructionOutOfBounds {
        function: FunctionID,
        instruction: usize,
    },
    #[error("Calling a builtin function failed")]
    CallFailed {
        function_name: StringIndex,
        #[source]
        inner: CallFailed,
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
        ($name:ident, |$machine:ident, $ctx:ident, $program:ident, mut $state:ident| $body:block) => {
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

                let $program = Program {
                    string_table: vec![
                        String::from("some string"),
                        String::from("global"),
                        String::from("some_function"),
                    ],
                    functions: vec![Function {
                        name: String::from("first"),
                        labels: vec![1, 2, 3],
                        body: Vec::new(),
                        return_ty: None,
                    }],
                    ..Default::default()
                };

                $body
            }
        };
    }

    ps_test!(return_from_last_stack_frame, |_mac, ctx, p, mut state| {
        let cont = state.execute(Instruction::Return, &p, ctx);

        assert_eq!(cont, Continuation::Halt);
        assert!(state.stack.is_empty());
    });

    ps_test!(
        return_from_penultimate_stack_frame,
        |_mac, ctx, p, mut state| {
            state.stack.push(StackFrame {
                function_id: 0,
                ip: 0,
            });

            let cont = state.execute(Instruction::Return, &p, ctx);

            assert_eq!(cont, Continuation::Continue);
            assert_eq!(state.stack.len(), 1);
        }
    );

    ps_test!(call_user_function_no_args, |_mac, ctx, p, mut state| {
        let instr = Instruction::CallUserFunction { function_id: 42 };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.stack.len(), 2);
        assert_eq!(
            state.stack.last().unwrap(),
            &StackFrame {
                function_id: 42,
                ip: 1
            }
        );
    });

    ps_test!(push_int, |_mac, ctx, p, mut state| {
        let instr = Instruction::PushInteger(42);

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Integer(42));
    });

    ps_test!(push_double, |_mac, ctx, p, mut state| {
        let instr = Instruction::PushDouble(3.14);

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Double(3.14));
    });

    ps_test!(push_bool, |_mac, ctx, p, mut state| {
        let instr = Instruction::PushBoolean(true);

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Boolean(true));
    });

    ps_test!(push_string_from_string_pool, |_mac, ctx, p, mut state| {
        let instr = Instruction::PushString { string_id: 0 };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::String(String::from("some string")));
    });

    ps_test!(push_unknown_string, |_mac, ctx, p, mut state| {
        let instr = Instruction::PushString { string_id: 42 };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(
            cont,
            Continuation::Fault(Fault::UnknownString { string_id: 42 })
        );
    });

    ps_test!(get_global_variable, |mac, ctx, p, mut state| {
        mac.store_global(&p.string_table[1], Value::Integer(42));
        let instr = Instruction::LoadGlobal { variable_id: 1 };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Integer(42));
    });

    ps_test!(set_global_variable, |mac, ctx, p, mut state| {
        let value = Value::Integer(42);
        state.values.push(value.clone());
        let instr = Instruction::StoreGlobal { variable_id: 1 };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert!(state.values.is_empty());
        let got = mac.load_global(&p.string_table[1]).unwrap();
        assert_eq!(got, value);
    });

    ps_test!(pop_from_stack, |mac, ctx, p, mut state| {
        state.values.push(Value::Integer(42));

        let cont = state.execute(Instruction::Pop, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert!(state.values.is_empty());
    });

    ps_test!(pop_from_empty_stack_is_error, |mac, ctx, p, mut state| {
        let cont = state.execute(Instruction::Pop, &p, ctx);

        assert_eq!(cont, Continuation::Fault(Fault::PoppedFromEmptyStack));
    });

    ps_test!(if_with_empty_stack_is_error, |mac, ctx, p, mut state| {
        let instr = Instruction::Branch {
            true_label: 0,
            false_label: 0,
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Fault(Fault::PoppedFromEmptyStack));
    });

    ps_test!(if_with_non_boolean_is_error, |mac, ctx, p, mut state| {
        let instr = Instruction::Branch {
            true_label: 0,
            false_label: 0,
        };
        state.values.push(Value::Integer(42));

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(
            cont,
            Continuation::Fault(Fault::TypeError {
                found: Type::Integer,
                expected: Type::Boolean,
            })
        );
    });

    ps_test!(take_true_branch, |mac, ctx, p, mut state| {
        let instr = Instruction::Branch {
            true_label: 1,
            false_label: 42,
        };
        state.values.push(Value::Boolean(true));

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        let sf = state.stack_frame().unwrap();
        let func = &p.functions[sf.function_id];
        assert_eq!(sf.ip, func.labels[1]);
    });

    ps_test!(take_false_branch, |mac, ctx, p, mut state| {
        let instr = Instruction::Branch {
            true_label: 1,
            false_label: 2,
        };
        state.values.push(Value::Boolean(false));

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        let sf = state.stack_frame().unwrap();
        let func = &p.functions[sf.function_id];
        assert_eq!(sf.ip, func.labels[2]);
    });

    ps_test!(goto_valid_label, |mac, ctx, p, mut state| {
        let instr = Instruction::Goto { label: 2 };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        let sf = state.stack_frame().unwrap();
        let func = &p.functions[sf.function_id];
        assert_eq!(sf.ip, func.labels[2]);
    });

    ps_test!(goto_invalid_label, |mac, ctx, p, mut state| {
        let instr = Instruction::Goto { label: 42 };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Fault(Fault::UnknownLabel(42)));
    });
}
