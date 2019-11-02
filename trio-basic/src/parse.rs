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
    use codespan_reporting::diagnostic::Diagnostic;
    use slog::Discard;
    use syntax::ParseError;

    struct PanicReporter;

    impl DiagnosticReporter for PanicReporter {
        fn on_parse_error(&self, error: &ParseError, _file: FileId) {
            panic!("{}", error);
        }

        fn on_diagnostic(&self, diag: Diagnostic, _file: FileId) {
            panic!("{:?}", diag);
        }
    }

    #[test]
    fn run_the_parse_system() {
        let logger = Logger::root(Discard, slog::o!());
        let mut files = Files::new();
        let file = files.add("main", "DIM x AS INTEGER");
        let mut world = World::new();
        world.insert(files);
        let files = &[file];
        let mut sys = Parse::new(files, &PanicReporter, logger.clone());
        RunNow::setup(&mut sys, &mut world);

        sys.run_now(&mut world);

        let ast_nodes: ReadStorage<Ast> = world.system_data();
        assert_eq!(ast_nodes.count(), 1);
        let got = ast_nodes.join().next().unwrap();
        assert_eq!(got.file, file);
    }
}
