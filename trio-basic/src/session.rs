use crate::{parse::Parse, DiagnosticReporter};
use codespan::{FileId, Files};
use slog::Logger;
use specs::{EntityBuilder, RunNow, World, WorldExt};

pub struct Session {
    world: World,
    logger: Logger,
}

impl Session {
    pub fn new(source_code: Files, logger: Logger) -> Self {
        let mut world = World::new();
        world.insert(source_code);

        Session { world, logger }
    }

    pub fn world(&self) -> &World { &self.world }

    pub fn create_entity(&mut self) -> EntityBuilder {
        self.world.create_entity()
    }

    pub fn parse(&mut self, files: &[FileId], diags: &dyn DiagnosticReporter) {
        let logger = self.logger.new(slog::o!("phase" => "parse"));
        let mut parse = Parse::new(files, diags, logger);

        parse.setup(&mut self.world);
        parse.run_now(&self.world);
    }
}
