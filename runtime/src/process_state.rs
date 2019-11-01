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

    pub fn at_function_entry(function: FunctionID) -> Self {
        ProcessState {
            stack: vec![StackFrame { function, ip: 0 }],
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
            Instruction::CallUserFunction { function } => {
                self.on_call_user_func(function, ctx)?;
            },
            Instruction::PushInteger(i) => self.values.push(Value::Integer(i)),
            Instruction::PushDouble(d) => self.values.push(Value::Double(d)),
            Instruction::PushBoolean(b) => self.values.push(Value::Boolean(b)),
            Instruction::PushString { string } => {
                let string = string
                    .lookup(&program.string_table)
                    .ok_or_else(|| Fault::UnknownString { string })?;
                self.values.push(Value::String(string.clone()));
            },
            Instruction::LoadGlobal { variable } => {
                self.on_load_global(variable, &program.string_table, ctx)?;
            },
            Instruction::StoreGlobal { variable } => {
                self.on_store_global(variable, &program.string_table, ctx)?;
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
            Instruction::CallBuiltinFunction { function, args } => {
                self.on_call_builtin(function, args, program, ctx)?
            },
        }

        self.advance_ip();
        Continuation::Continue
    }

    fn on_call_builtin(
        &mut self,
        function: StringIndex,
        arg_count: usize,
        program: &Program,
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        if self.values.len() < arg_count {
            return Err(Fault::PoppedFromEmptyStack);
        }

        let arg_start = self.values.len() - arg_count;
        let args = &self.values[arg_start..];

        let function_name = function
            .lookup(&program.string_table)
            .ok_or_else(|| Fault::UnknownString { string: function })?;

        slog::debug!(ctx.logger, "Calling a builtin function";
            "name" => function_name,
            "args" => format_args!("{:?}", args));

        let result = ctx.machine.call(function_name, args).map_err(|e| {
            Fault::CallFailed {
                function_name: function,
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
        label: LabelIndex,
        current_function: &Function,
    ) -> Result<(), Fault> {
        let mut sf = self.stack_frame_mut().expect("unreachable");
        sf.ip = label
            .lookup(&current_function.labels)
            .copied()
            .ok_or_else(|| Fault::UnknownLabel(label))?;

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

        let label = if condition { true_label } else { false_label };

        self.on_goto(label, current_function)
    }

    fn on_store_global(
        &mut self,
        string: StringIndex,
        strings: &[String],
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        let value = self
            .values
            .pop()
            .ok_or_else(|| Fault::PoppedFromEmptyStack)?;

        let variable_name = string
            .lookup(&strings)
            .ok_or_else(|| Fault::UnknownString { string })?;

        ctx.machine.store_global(variable_name, value);

        Ok(())
    }

    fn on_load_global(
        &mut self,
        string: StringIndex,
        strings: &[String],
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        let variable_name = string
            .lookup(&strings)
            .ok_or_else(|| Fault::UnknownString { string })?;

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
        function: FunctionID,
        ctx: Ctx<'_>,
    ) -> Result<(), Fault> {
        self.stack.push(StackFrame { function, ip: 0 });
        slog::trace!(ctx.logger, "Calling a user function";
                    "function-id" => function,
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
        let StackFrame { function, .. } = self.stack_frame()?;

        function
            .lookup(&program.functions)
            .ok_or_else(|| Fault::UnknownFunction(function))
    }

    fn fetch(
        &self,
        program: &Program,
        logger: &Logger,
    ) -> Result<Instruction, Fault> {
        let StackFrame { function, ip } = self.stack_frame()?;

        let instruction = self
            .current_function(program)?
            .body
            .get(ip)
            .copied()
            .ok_or_else(|| Fault::InstructionOutOfBounds {
                function,
                instruction: ip,
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
    pub function: FunctionID,
    /// The [`Instruction`] index within the current [`Function`]'s body.
    pub ip: usize,
}

impl slog::KV for StackFrame {
    fn serialize(
        &self,
        record: &slog::Record,
        ser: &mut dyn slog::Serializer,
    ) -> slog::Result {
        slog::Value::serialize(&self.function, record, "function", ser)?;
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
    #[error("No such string with ID {string}")]
    UnknownString { string: StringIndex },
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
    use std::sync::{Arc, Mutex};

    /// A helper for setting up a simple [`Program`] and all the inputs
    /// needed by [`ProcessState::step()`].
    macro_rules! ps_test {
        ($name:ident, |$machine:ident, $ctx:ident, $program:ident, mut $state:ident| $body:block) => {
            #[test]
            fn $name() {
                let logger = Logger::root(Discard, slog::o!());
                let $machine = Arc::new(BasicMachine::default());
                let mut $state = ProcessState::default();
                $state.stack.push(StackFrame {
                    function: FunctionID(0),
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
                function: FunctionID(0),
                ip: 0,
            });

            let cont = state.execute(Instruction::Return, &p, ctx);

            assert_eq!(cont, Continuation::Continue);
            assert_eq!(state.stack.len(), 1);
        }
    );

    ps_test!(call_user_function_no_args, |_mac, ctx, p, mut state| {
        let instr = Instruction::CallUserFunction {
            function: FunctionID(42),
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.stack.len(), 2);
        assert_eq!(
            state.stack.last().unwrap(),
            &StackFrame {
                function: FunctionID(42),
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
        let instr = Instruction::PushString {
            string: StringIndex(0),
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::String(String::from("some string")));
    });

    ps_test!(push_unknown_string, |_mac, ctx, p, mut state| {
        let instr = Instruction::PushString {
            string: StringIndex(42),
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(
            cont,
            Continuation::Fault(Fault::UnknownString {
                string: StringIndex(42)
            })
        );
    });

    ps_test!(get_global_variable, |mac, ctx, p, mut state| {
        mac.store_global(&p.string_table[1], Value::Integer(42));
        let instr = Instruction::LoadGlobal {
            variable: StringIndex(1),
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        assert_eq!(state.values.len(), 1);
        assert_eq!(state.values[0], Value::Integer(42));
    });

    ps_test!(set_global_variable, |mac, ctx, p, mut state| {
        let value = Value::Integer(42);
        state.values.push(value.clone());
        let instr = Instruction::StoreGlobal {
            variable: StringIndex(1),
        };

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
            true_label: LabelIndex(0),
            false_label: LabelIndex(0),
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Fault(Fault::PoppedFromEmptyStack));
    });

    ps_test!(if_with_non_boolean_is_error, |mac, ctx, p, mut state| {
        let instr = Instruction::Branch {
            true_label: LabelIndex(0),
            false_label: LabelIndex(0),
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
            true_label: LabelIndex(1),
            false_label: LabelIndex(42),
        };
        state.values.push(Value::Boolean(true));

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        let sf = state.stack_frame().unwrap();
        let func = &p.functions[sf.function];
        assert_eq!(sf.ip, func.labels[1]);
    });

    ps_test!(take_false_branch, |mac, ctx, p, mut state| {
        let instr = Instruction::Branch {
            true_label: LabelIndex(1),
            false_label: LabelIndex(2),
        };
        state.values.push(Value::Boolean(false));

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        let sf = state.stack_frame().unwrap();
        let func = &p.functions[sf.function];
        assert_eq!(sf.ip, func.labels[2]);
    });

    ps_test!(goto_valid_label, |mac, ctx, p, mut state| {
        let instr = Instruction::Goto {
            label: LabelIndex(2),
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(cont, Continuation::Continue);
        let sf = state.stack_frame().unwrap();
        let func = &p.functions[sf.function];
        assert_eq!(sf.ip, func.labels[2]);
    });

    ps_test!(goto_invalid_label, |mac, ctx, p, mut state| {
        let instr = Instruction::Goto {
            label: LabelIndex(42),
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(
            cont,
            Continuation::Fault(Fault::UnknownLabel(LabelIndex(42)))
        );
    });

    ps_test!(
        call_a_builtin_function_with_no_args_and_returning_integer,
        |mac, ctx, p, mut state| {
            let expected = Value::Double(3.14);
            let expected_2 = expected.clone();
            mac.register_function("some_function", move |args| {
                assert!(args.is_empty());
                Ok(Some(expected_2.clone()))
            });
            let instr = Instruction::CallBuiltinFunction {
                function: StringIndex(2),
                args: 0,
            };

            let cont = state.execute(instr, &p, ctx);

            assert_eq!(cont, Continuation::Continue);
            assert_eq!(state.values.len(), 1);
            assert_eq!(state.values[0], expected);
        }
    );

    ps_test!(
        call_a_builtin_function_with_insufficient_args,
        |mac, ctx, p, mut state| {
            let instr = Instruction::CallBuiltinFunction {
                function: StringIndex(2),
                args: 3,
            };

            let cont = state.execute(instr, &p, ctx);

            assert_eq!(cont, Continuation::Fault(Fault::PoppedFromEmptyStack));
        }
    );

    ps_test!(call_unknown_function, |mac, ctx, p, mut state| {
        let instr = Instruction::CallBuiltinFunction {
            function: StringIndex(1),
            args: 0,
        };

        let cont = state.execute(instr, &p, ctx);

        assert_eq!(
            cont,
            Continuation::Fault(Fault::CallFailed {
                function_name: StringIndex(1),
                inner: CallFailed::UnknownFunction,
            })
        );
    });

    ps_test!(
        call_builtin_which_returned_an_error,
        |mac, ctx, p, mut state| {
            let err = CallFailed::Custom("Oops...");
            mac.register_function("some_function", move |_| Err(err));
            let instr = Instruction::CallBuiltinFunction {
                function: StringIndex(2),
                args: 0,
            };

            let cont = state.execute(instr, &p, ctx);

            assert_eq!(
                cont,
                Continuation::Fault(Fault::CallFailed {
                    function_name: StringIndex(2),
                    inner: err,
                })
            );
        }
    );

    ps_test!(
        call_a_builtin_function_with_3_args,
        |mac, ctx, p, mut state| {
            let got_args = Arc::new(Mutex::new(Vec::new()));
            let c2 = Arc::clone(&got_args);
            mac.register_function("some_function", move |args| {
                c2.lock().unwrap().extend(args.iter().cloned());
                Ok(None)
            });
            let instr = Instruction::CallBuiltinFunction {
                function: StringIndex(2),
                args: 3,
            };
            let expected = vec![
                Value::Boolean(true),
                Value::Integer(42),
                Value::Double(3.14),
            ];
            state.values.extend(expected.iter().cloned());

            let cont = state.execute(instr, &p, ctx);

            assert_eq!(cont, Continuation::Continue);
            let got_args = got_args.lock().unwrap();
            assert_eq!(*got_args, expected);
        }
    );
}
