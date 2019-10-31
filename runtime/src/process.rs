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
