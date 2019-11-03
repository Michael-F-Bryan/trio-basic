use crate::{
    lowering::Lowering, parse::Parse, source_code::LoadSourceCode,
    DiagnosticReporter,
};
use codespan::{FileId, Files};
use slog::Logger;
use specs::{EntityBuilder, RunNow, System, World, WorldExt};
use std::fmt::{self, Debug, Formatter};

/// A parsing session, containing all the information and data structures
/// related to the compilation process.
pub struct Session {
    world: World,
    logger: Logger,
}

impl Session {
    pub fn new(logger: Logger) -> Self {
        Session {
            world: World::new(),
            logger,
        }
    }

    pub fn world(&self) -> &World { &self.world }

    pub fn create_entity(&mut self) -> EntityBuilder<'_> {
        self.world.create_entity()
    }

    /// Adds all files in the project to the compilation session.
    pub fn load_source_code(&mut self, files: &[FileId], source_code: &Files) {
        let logger =
            self.logger.new(slog::o!("phase" => "loading-source-code"));
        self.run_pass(LoadSourceCode::new(files, source_code, logger));
        self.world.maintain();
    }

    /// Execute the parsing pass.
    pub fn parse(&mut self, diags: &dyn DiagnosticReporter) {
        let logger = self.logger.new(slog::o!("phase" => "parse"));
        self.run_pass(Parse::new(diags, logger));
        self.world.maintain();
    }

    pub fn lower(&mut self, diags: &dyn DiagnosticReporter) {
        let logger = self.logger.new(slog::o!("phase" => "lowering"));
        self.run_pass(Lowering::new(diags, logger));
        self.world.maintain();
    }

    fn run_pass<'a, S>(&'a mut self, mut pass: S)
    where
        S: System<'a>,
    {
        slog::debug!(self.logger, "Setting up a pass";
            "name" => std::any::type_name::<S>());
        pass.setup(&mut self.world);

        slog::debug!(self.logger, "Running a pass";
            "name" => std::any::type_name::<S>());
        pass.run_now(&self.world);
    }
}

impl Debug for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Session").finish()
    }
}
