//! The compiler pass in charge of parsing source code.

use crate::{source_code::File, DiagnosticReporter, Location};
use slog::Logger;
use specs::{prelude::*, Component};
use std::fmt::{self, Debug, Formatter};
use syntax::ast;

/// The system in charge of parsing.
pub struct Parse<'a> {
    logger: Logger,
    diags: &'a dyn DiagnosticReporter,
}

impl<'a> Parse<'a> {
    pub fn new(diags: &'a dyn DiagnosticReporter, logger: Logger) -> Self {
        Parse { diags, logger }
    }
}

impl<'sys, 'a: 'sys> System<'sys> for Parse<'a> {
    type SystemData = (
        WriteStorage<'sys, Ast>,
        ReadStorage<'sys, File>,
        WriteStorage<'sys, Location>,
        Entities<'sys>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ast_storage, files, mut locations, entities) = data;

        for (file, ent) in (&files, &entities).join() {
            let location = locations
                .get(ent)
                .copied()
                .expect("All source files have a location");

            slog::debug!(self.logger, "Parsing a file";
                "name" => &file.name,
                "length" => file.src.len());

            match syntax::parse(&file.src) {
                Ok(root) => {
                    entities
                        .build_entity()
                        .with(Ast { root }, &mut ast_storage)
                        .with(location, &mut locations)
                        .build();
                },
                Err(e) => self.diags.on_parse_error(&e, location.file),
            }
        }
    }
}

impl<'a> Debug for Parse<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Parse {
            ref logger,
            diags: _,
        } = self;

        f.debug_struct("Parse").field("logger", logger).finish()
    }
}

/// The top-level node containing the AST for a single file.
#[derive(Debug, Clone, PartialEq, specs::Component)]
#[storage(VecStorage)]
pub struct Ast {
    pub root: ast::File,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::{PanicReporter, Recorder},
        source_code::LoadSourceCode,
    };
    use codespan::{FileId, Files};
    use slog::Discard;

    macro_rules! system_test {
        ($name:ident, $($filename:expr => $src:expr),*;
            |$world:ident, $files:ident, $logger:ident| $body:block) => {
            #[test]
            fn $name() {
                let $logger = Logger::root(Discard, slog::o!());
                let mut $world = World::new();

                // "load" our source code into memory
                let mut source_code = Files::new();

                let mut $files = Vec::<FileId>::new();
                $(
                    let file_id = source_code.add($filename, $src);
                    $files.push(file_id);
                )*
                let mut load_src = LoadSourceCode::new(&$files, &source_code, $logger.clone());
                RunNow::setup(&mut load_src, &mut $world);
                RunNow::run_now(&mut load_src, &mut $world);

                $body;
            }
        };
    }

    system_test! {
        run_the_parse_system, "main" => "DIM x AS INTEGER";
        |world, files, logger| {
            let mut sys = Parse::new(&PanicReporter, logger.clone());
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
            let mut sys = Parse::new(&diags, logger.clone());
            RunNow::setup(&mut sys, &mut world);

            sys.run_now(&mut world);

            assert!(diags.diagnostics.lock().unwrap().is_empty());
            let parse_errors = diags.parse_errors.lock().unwrap();
            assert_eq!(parse_errors.len(), 1);
            assert_eq!(parse_errors[0].0, files[0]);
        }
    }
}
