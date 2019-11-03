//! The TRIO Basic Compiler.

#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    rust_2018_idioms
)]

pub mod diagnostics;
pub mod lowering;
pub mod parse;
mod session;
mod source_code;

pub use diagnostics::DiagnosticReporter;
pub use session::Session;

use codespan::{FileId, Files, Span};
use slog::Logger;
use specs::{prelude::*, Component};

/// The entrypoint to the compiler.
pub fn compile<C, D>(
    project: Project,
    logger: Logger,
    cb: &mut dyn Callback,
    diagnostics: &dyn DiagnosticReporter,
) {
    let mut session = Session::new(logger.clone());

    session.load_source_code(&project.files, &project.source_code);

    session.parse(diagnostics);
    if cb.after_parsing(&session) == Compilation::Halt {
        return;
    }

    session.lower(diagnostics);
    if cb.after_lowering(&session) == Compilation::Halt {
        return;
    }
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
    fn after_parsing(&mut self, _session: &Session) -> Compilation {
        Compilation::Continue
    }

    fn after_lowering(&mut self, _session: &Session) -> Compilation {
        Compilation::Continue
    }
}

/// Control flow for the compilation process.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Compilation {
    Continue,
    Halt,
}

/// The location of something in source code.
#[derive(Debug, Copy, Clone, PartialEq, specs::Component)]
#[storage(VecStorage)]
pub struct Location {
    pub file: FileId,
    pub span: Span,
}
