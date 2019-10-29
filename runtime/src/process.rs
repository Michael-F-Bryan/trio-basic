use crate::{Machine, NopMachine, ProcessState};
use slog::{Discard, Logger};
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
}
