use codespan::FileId;
use codespan_reporting::diagnostic::Diagnostic;
use std::sync::Mutex;
use syntax::ParseError;

pub trait DiagnosticReporter {
    fn on_parse_error(&self, _error: &ParseError, _file: FileId) {}
    fn on_diagnostic(&self, _diag: Diagnostic, _file: FileId) {}
}

/// A [`DiagnosticReporter`] which will panic on the first diagnostic message.
///
/// Mainly for use during testing.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PanicReporter;

impl DiagnosticReporter for PanicReporter {
    fn on_parse_error(&self, error: &ParseError, _file: FileId) {
        panic!("Parse error: {}", error);
    }

    fn on_diagnostic(&self, diag: Diagnostic, _file: FileId) {
        panic!("{:?}", diag);
    }
}

/// A [`DiagnosticReporter`] which will record all diagnostics for use at a
/// later time.
#[derive(Debug, Default)]
pub struct Recorder {
    pub diagnostics: Mutex<Vec<(FileId, Diagnostic)>>,
    pub parse_errors: Mutex<Vec<(FileId, ParseError)>>,
}

impl DiagnosticReporter for Recorder {
    fn on_parse_error(&self, error: &ParseError, file: FileId) {
        self.parse_errors
            .lock()
            .unwrap()
            .push((file, error.clone()));
    }

    fn on_diagnostic(&self, diag: Diagnostic, file: FileId) {
        self.diagnostics.lock().unwrap().push((file, diag));
    }
}
