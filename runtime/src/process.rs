use crate::{Continuation, Ctx, Fault, Machine, ProcessState};
use bytecode::Program;
use slog::Logger;
use std::sync::Arc;

pub struct Process {
    pub(crate) logger: Logger,
    pub(crate) machine: Arc<dyn Machine>,
    pub(crate) state: ProcessState,
}

impl Process {
    pub fn new(logger: Logger, machine: Arc<dyn Machine>) -> Self {
        Process {
            logger,
            machine,
            state: ProcessState::default(),
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), Fault> {
        // to get things started we'll manually call the main function
        self.state.call_user_func(program.entrypoint)?;

        loop {
            let ctx = Ctx::new(&self.logger, &*self.machine);

            match self.state.step(program, ctx) {
                Continuation::Continue => continue,
                Continuation::Halt => return Ok(()),
                Continuation::Fault(fault) => return Err(fault),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BasicMachine, Value};
    use bytecode::{Function, FunctionIndex, Instruction};
    use slog::Discard;

    #[test]
    fn run_a_program_to_add_two_numbers() {
        let program = Program {
            entrypoint: FunctionIndex(0),
            functions: vec![Function {
                name: String::from("main"),
                body: vec![
                    Instruction::PushInteger(1),
                    Instruction::PushInteger(2),
                    Instruction::Add,
                    Instruction::Return,
                ],
                ..Default::default()
            }],
            string_table: Vec::new(),
        };
        let mut process = Process::new(
            Logger::root(Discard, slog::o!()),
            Arc::new(BasicMachine::default()),
        );

        process.run(&program).unwrap();

        assert_eq!(process.state.values.len(), 1);
        assert!(process.state.stack.is_empty());
        assert_eq!(process.state.values[0], Value::Integer(3));
    }
}
