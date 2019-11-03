use crate::Location;
use codespan::{FileId, Files};
use slog::Logger;
use specs::{prelude::*, Component};

/// A [`System`] for loading source code into the [`World`] for later use by the
/// parser.
pub struct LoadSourceCode<'src> {
    files: &'src [FileId],
    source_code: &'src Files,
    logger: Logger,
}

impl<'src> LoadSourceCode<'src> {
    pub(crate) fn new(
    files: &'src [FileId],
    source_code: &'src Files,
        logger: Logger,
    ) -> Self {
        LoadSourceCode {
            files,
            source_code,
            logger,
        }
    }
}

impl<'sys, 'src: 'sys> System<'sys> for LoadSourceCode<'src> {
    type SystemData = (
        WriteStorage<'sys, File>,
        WriteStorage<'sys, Location>,
        Entities<'sys>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut files, mut locations, entities) = data;

        for file_id in self.files.iter().copied() {
            let src = self.source_code.source(file_id).to_string();
            let name = self.source_code.name(file_id).to_string();
            let span = self.source_code.source_span(file_id);

            slog::debug!(self.logger, "Loading a source file";
                "name" => &name,
                "file-id" => format_args!("{:?}", file_id),
                "length" => src.len());

            entities
                .build_entity()
                .with(
                    Location {
                        file: file_id,
                        span,
                    },
                    &mut locations,
                )
                .with(File { name, src }, &mut files)
                .build();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
#[storage(VecStorage)]
pub struct File {
    pub name: String,
    pub src: String,
}
