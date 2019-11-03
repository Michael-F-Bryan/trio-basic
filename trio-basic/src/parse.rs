//! The compiler pass in charge of parsing source code.

use crate::DiagnosticReporter;
use codespan::{FileId, Files, Span};
use slog::Logger;
use specs::{prelude::*, Component};
use std::fmt::{self, Debug, Formatter};
use syntax::ast::File;

/// The system in charge of parsing.
pub struct Parse<'a> {
    logger: Logger,
    files: &'a [FileId],
    diags: &'a dyn DiagnosticReporter,
}

impl<'a> Parse<'a> {
    pub fn new(
        files: &'a [FileId],
        diags: &'a dyn DiagnosticReporter,
        logger: Logger,
    ) -> Self {
        Parse {
            files,
            diags,
            logger,
        }
    }
}

impl<'a> System<'a> for Parse<'a> {
    type SystemData = (
        WriteStorage<'a, Ast>,
        WriteStorage<'a, Location>,
        ReadExpect<'a, Files>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ast_storage, mut locations, files, entities) = data;

        for &file in self.files {
            let src = files.source(file);
            slog::debug!(self.logger, "Parsing a file";
                "name" => files.name(file),
                "length" => src.len());

            match File::from_str(src) {
                Ok(root) => {
                    let span = files.source_span(file);
                    entities
                        .build_entity()
                        .with(Ast { root }, &mut ast_storage)
                        .with(Location { file, span }, &mut locations)
                        .build();
                },
                Err(e) => self.diags.on_parse_error(&e, file),
            }
        }
    }
}

impl<'a> Debug for Parse<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Parse {
            ref files,
            ref logger,
            diags: _,
        } = self;

        f.debug_struct("Parse")
            .field("files", files)
            .field("logger", logger)
            .finish()
    }
}

/// The top-level node containing the AST for a single file.
#[derive(Debug, Clone, PartialEq, specs::Component)]
#[storage(VecStorage)]
pub struct Ast {
    pub root: File,
}

/// The location of something in source code.
#[derive(Debug, Copy, Clone, PartialEq, specs::Component)]
#[storage(VecStorage)]
pub struct Location {
    pub file: FileId,
    pub span: Span,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::{PanicReporter, Recorder};
    use slog::Discard;

    macro_rules! system_test {
        ($name:ident, $($filename:expr => $src:expr),*;
            |$world:ident, $files:ident, $logger:ident| $body:block) => {
            #[test]
            fn $name() {
                let $logger = Logger::root(Discard, slog::o!());
                let mut source_code = Files::new();

                let mut $files = Vec::<FileId>::new();
                $(
                    let file_id = source_code.add($filename, $src);
                    $files.push(file_id);
                )*

                let mut $world = World::new();
                $world.insert(source_code);

                $body;
            }
        };
    }

    system_test! {
        run_the_parse_system, "main" => "DIM x AS INTEGER";
        |world, files, logger| {
            let mut sys = Parse::new(&files, &PanicReporter, logger.clone());
            RunNow::setup(&mut sys, &mut world);

            sys.run_now(&mut world);

            let (ast, locations): (ReadStorage<'_, Ast>, ReadStorage<'_, Location>) = world.system_data();
            let got: Vec<(&Ast, &Location)> = (&ast, &locations).join().collect();
            assert_eq!(got.len(), 1);
            assert_eq!(got[0].1.file, files[0]);
        }
    }

    system_test! {
        detect_parse_failure, "main" => "DIM x";
        |world, files, logger| {
            let diags = Recorder::default();
            let mut sys = Parse::new(&files, &diags, logger.clone());
            RunNow::setup(&mut sys, &mut world);

            sys.run_now(&mut world);

            assert!(diags.diagnostics.lock().unwrap().is_empty());
            let parse_errors = diags.parse_errors.lock().unwrap();
            assert_eq!(parse_errors.len(), 1);
            assert_eq!(parse_errors[0].0, files[0]);
        }
    }
}
