use crate::Location;
use codespan::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::sync::Mutex;
use syntax::ParseError;

pub trait DiagnosticReporter {
    fn on_parse_error(&self, _error: &ParseError, _file: FileId) {}
    fn on_diagnostic(&self, _diag: Diagnostic) {}
}

/// A helper trait for commonly-used diagnostics.
pub trait DiagnosticReporterExt {
    fn duplicate_definition(
        &self,
        name: &str,
        original: Location,
        duplicate: Location,
    );
}

impl<'a> DiagnosticReporterExt for &'a dyn DiagnosticReporter {
    fn duplicate_definition(
        &self,
        name: &str,
        original: Location,
        duplicate: Location,
    ) {
        let primary_label =
            Label::new(duplicate.file, duplicate.span, "Duplicate definition");
        let original_label =
            Label::new(original.file, original.span, "Original definition");

        let msg = format!("\"{}\" is already defined", name);

        let diag = Diagnostic::new_error(msg, primary_label)
            .with_secondary_labels(vec![original_label]);

        self.on_diagnostic(diag);
    }
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

    fn on_diagnostic(&self, diag: Diagnostic) {
        panic!("{:?}", diag);
    }
}

/// A [`DiagnosticReporter`] which will record all diagnostics for use at a
/// later time.
#[derive(Debug, Default)]
pub struct Recorder {
    pub diagnostics: Mutex<Vec<Diagnostic>>,
    pub parse_errors: Mutex<Vec<(FileId, ParseError)>>,
}

impl DiagnosticReporter for Recorder {
    fn on_parse_error(&self, error: &ParseError, file: FileId) {
        self.parse_errors
            .lock()
            .unwrap()
            .push((file, error.clone()));
    }

    fn on_diagnostic(&self, diag: Diagnostic) {
        self.diagnostics.lock().unwrap().push(diag);
    }
}
