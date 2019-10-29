use crate::{Machine, NopMachine};
use slog::{Discard, Logger};
use std::sync::Arc;

pub struct Process {
    pub(crate) logger: Logger,
    pub(crate) machine: Arc<dyn Machine>,
}

impl Process {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

#[derive(Clone)]
pub struct Builder {
    logger: Logger,
    machine: Arc<dyn Machine>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            logger: Logger::root(Discard, slog::o!()),
            machine: Arc::new(NopMachine),
        }
    }

    pub fn build(&mut self) -> Process {
        let Builder { logger, machine } = self.clone();

        Process { logger, machine }
    }
}

impl Default for Builder {
    fn default() -> Builder {
        Builder::new()
    }
}
