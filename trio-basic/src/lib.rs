//! The TRIO Basic Compiler.

pub mod diagnostics;
pub mod parse;
mod session;

pub use diagnostics::DiagnosticReporter;
pub use session::Session;

use codespan::{FileId, Files};
use slog::Logger;

/// The entrypoint to the compiler.
pub fn compile<C, D>(
    project: Project,
    logger: Logger,
    cb: &mut dyn Callback,
    diagnostics: &dyn DiagnosticReporter,
) {
    let mut session = Session::new(project.source_code, logger.clone());

    session.parse(&project.files, diagnostics);
    if cb.after_parsing(&session) == Compilation::Halt {
        return;
    }

    unimplemented!()
}

/// The source code of a project after it is read into memory.
#[derive(Debug, Clone)]
pub struct Project {
    pub files: Vec<FileId>,
    pub source_code: Files,
}

/// Callbacks that can be used to see the progress of the compilation process.
pub trait Callback {
    /// Called after the source code is parsed.
    fn after_parsing(&self, _session: &Session) -> Compilation {
        Compilation::Continue
    }
}

/// Control flow for the compilation process.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Compilation {
    Continue,
    Halt,
}
