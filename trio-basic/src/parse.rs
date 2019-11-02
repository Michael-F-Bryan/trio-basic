//! The compiler pass in charge of parsing source code.

use crate::DiagnosticReporter;
use codespan::{FileId, Files};
use slog::Logger;
use specs::{prelude::*, Component};
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
    type SystemData =
        (WriteStorage<'a, Ast>, ReadExpect<'a, Files>, Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut ast_storage, files, entities) = data;

        for &file in self.files {
            let src = files.source(file);
            slog::debug!(self.logger, "Parsing a file";
                "name" => files.name(file),
                "length" => src.len());

            match File::from_str(src) {
                Ok(root) => {
                    let ast = Ast { root, file };
                    entities.build_entity().with(ast, &mut ast_storage).build();
                },
                Err(e) => self.diags.on_parse_error(&e, file),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, specs::Component)]
#[storage(VecStorage)]
pub struct Ast {
    pub root: File,
    pub file: FileId,
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

            let ast_nodes: ReadStorage<Ast> = world.system_data();
            assert_eq!(ast_nodes.count(), 1);
            let got = ast_nodes.join().next().unwrap();
            assert_eq!(got.file, files[0]);
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
