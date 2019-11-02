//! The TRIO Basic Compiler.

use codespan::{FileId, Files};
use codespan_reporting::diagnostic::Diagnostic;
use slog::Logger;

/// The source code of a project after it is read into memory.
#[derive(Debug, Clone)]
pub struct Project {
    pub files: Vec<FileId>,
    pub source_code: Files,
}

/// The entrypoint to the compiler.
pub fn compile<C, D>(
    project: Project,
    logger: Logger,
    cb: &mut C,
    diagnostics: &D,
) -> Outcome
where
    C: Callback,
    D: DiagnosticReporter,
{
    unimplemented!()
}

/// Callbacks that can be used to see the progress of the compilation process.
pub trait Callback {
    /// Called after the source code is parsed.
    fn after_parsing(&self) {}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Outcome {
    Completed,
    Failed,
}

pub trait DiagnosticReporter {
    fn on_diagnostic(&self, diag: Diagnostic, file: FileId, files: &Files) {}
}
