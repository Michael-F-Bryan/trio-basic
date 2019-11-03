use crate::{
    parse::{Ast, },
    DiagnosticReporter,
Location,
};
use slog::Logger;
use specs::{prelude::*, Component};
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

/// Turns an AST into a more usable graph of [`BasicBlock`]s.
pub struct Lowering<'a> {
    pub(crate) logger: Logger,
    pub(crate) diags: &'a dyn DiagnosticReporter,
}

impl<'a> Lowering<'a> {
    pub(crate) fn new(
        diags: &'a dyn DiagnosticReporter,
        logger: Logger,
    ) -> Self {
        Lowering { logger, diags }
    }
}

impl<'sys, 'a: 'sys> System<'sys> for Lowering<'a> {
    type SystemData = (
        ReadStorage<'sys, Ast>,
        WriteStorage<'sys, Program>,
        WriteStorage<'sys, Location>,
        Entities<'sys>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ast_nodes, programs, mut locations, entities) = data;

        for (ast, ent) in (&ast_nodes, &entities).join() {
            let location = locations.get(ent).copied().unwrap();

            entities
                .build_entity()
                .with(location, &mut locations)
                .build();
        }
    }
}

impl<'a> Debug for Lowering<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Lowering {
            ref logger,
            diags: _,
        } = self;

        f.debug_struct("Lowering").field("logger", logger).finish()
    }
}

struct InstructionBlockBuilder<'a> {
    entities: Entities<'a>,
    locations: &'a mut WriteStorage<'a, Location>,
    variables: HashMap<String, Entity>,
}

impl<'a> Debug for InstructionBlockBuilder<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let InstructionBlockBuilder {
            ref variables,
            entities: _,
            locations: _,
        } = self;

        f.debug_struct("InstructionBlockBuilder")
            .field("variables", variables)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Program {
    pub entrypoint: Entity,
    pub variables: Vec<Entity>,
    pub basic_blocks: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct BasicBlock {
    pub label: Option<Entity>,
    pub instructions: Vec<Entity>,
}
