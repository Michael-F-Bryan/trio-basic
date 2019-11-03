use crate::{
    diagnostics::{DiagnosticReporter, DiagnosticReporterExt},
    parse::Ast,
    Location,
};
use codespan::FileId;
use slog::Logger;
use specs::{prelude::*, Component};
use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};
use syntax::ast::{self, AstNode, Dim, Line};

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
        WriteStorage<'sys, Variable>,
        WriteStorage<'sys, Identifier>,
        WriteStorage<'sys, Location>,
        Entities<'sys>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            ast_nodes,
            mut programs,
            mut variables,
            mut identifiers,
            mut locations,
            entities,
        ) = data;

        for (ast, ent) in (&ast_nodes, &entities).join() {
            let location = locations.get(ent).copied().unwrap();

            let program = lower(
                &ast.root,
                location,
                &entities,
                &mut locations,
                &mut variables,
                &mut identifiers,
                self.diags,
                self.logger
                    .new(slog::o!("file-id" => format!("{:?}", location.file))),
            );

            entities
                .build_entity()
                .with(program, &mut programs)
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

/// Converts a single [`syntax::ast::File`] into a [`Program`].
pub fn lower<'sys>(
    node: &syntax::ast::File,
    node_location: Location,
    entities: &Entities<'sys>,
    locations: &mut WriteStorage<'sys, Location>,
    variables: &mut WriteStorage<'sys, Variable>,
    identifiers: &mut WriteStorage<'sys, Identifier>,
    diags: &dyn DiagnosticReporter,
    logger: Logger,
) -> Program {
    let mut builder = ProgramBuilder::new(
        entities,
        locations,
        variables,
        identifiers,
        node_location.file,
        diags,
        logger,
    );

    for line in &node.lines {
        match line {
            Line::Dim(ref dim) => builder.dim(dim),
            _ => unimplemented!(),
        }
    }

    unimplemented!()
}

struct ProgramBuilder<'a, 'sys> {
    file: FileId,
    variable_declarations: HashMap<String, Entity>,
    logger: Logger,
    diags: &'a dyn DiagnosticReporter,

    entities: &'a Entities<'sys>,
    locations: &'a mut WriteStorage<'sys, Location>,
    variables: &'a mut WriteStorage<'sys, Variable>,
    identifiers: &'a mut WriteStorage<'sys, Identifier>,
}

impl<'a, 'sys> ProgramBuilder<'a, 'sys> {
    fn new(
        entities: &'a Entities<'sys>,
        locations: &'a mut WriteStorage<'sys, Location>,
        variables: &'a mut WriteStorage<'sys, Variable>,
        identifiers: &'a mut WriteStorage<'sys, Identifier>,
        file: FileId,
        diags: &'a dyn DiagnosticReporter,
        logger: Logger,
    ) -> Self {
        ProgramBuilder {
            entities,
            locations,
            diags,
            file,
            identifiers,
            logger,
            variables,
            variable_declarations: HashMap::new(),
        }
    }

    /// Declare a variable.
    fn dim(&mut self, dim: &Dim) {
        let ident = self.insert_ident(&dim.name);
        let ty = self.insert_ident(&dim.ty);
        let location = Location {
            file: self.file,
            span: dim.span(),
        };

        let normalised_name = dim.name.name.to_lowercase();

        use std::collections::hash_map::Entry;

        match self.variable_declarations.entry(normalised_name) {
            Entry::Occupied(occupied) => {
                // the variable already exists, emit an error
                let original_location = self
                    .locations
                    .get(*occupied.get())
                    .copied()
                    .expect("All dims have a location");
                self.diags.duplicate_definition(
                    &dim.name.name,
                    original_location,
                    location,
                );
            },
            Entry::Vacant(vacant) => {
                // otherwise, create it
                let ent = self
                    .entities
                    .build_entity()
                    .with(location, &mut self.locations)
                    .with(Variable { name: ident, ty }, &mut self.variables)
                    .build();

                slog::trace!(self.logger, "Declared a variable";
                    "name" => &dim.name.name,
                    "type" => &dim.ty.name,
                    "span" => format_args!("{:?}", dim.span()),
                    "entity" => format_args!("{:?}", ent));

                vacant.insert(ent);
            },
        }
    }

    fn insert_ident(&mut self, ident: &ast::Ident) -> Entity {
        let ent = self
            .entities
            .build_entity()
            .with(
                Identifier {
                    name: ident.name.to_string(),
                },
                &mut self.identifiers,
            )
            .with(
                Location {
                    file: self.file,
                    span: ident.span(),
                },
                &mut self.locations,
            )
            .build();

        ent
    }
}

impl<'a, 'sys> Debug for ProgramBuilder<'a, 'sys> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ProgramBuilder {
            ref variable_declarations,
            ref logger,
            file,

            variables: _,
            identifiers: _,
            entities: _,
            locations: _,
            diags: _,
        } = self;

        f.debug_struct("InstructionBlockBuilder")
            .field("file", &file)
            .field("logger", logger)
            .field("variable_declarations", variable_declarations)
            .finish()
    }
}

/// A series of [`BasicBlock`]s with an entrypoint and some [`Variable`]s.
#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Program {
    /// The first [`BasicBlock`].
    pub entrypoint: Entity,
    pub variables: Vec<Entity>,
    pub basic_blocks: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct BasicBlock {
    pub label: Option<Entity>,
    pub instructions: Vec<Entity>,
    pub next: NextBasicBlock,
}

#[derive(Debug, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Identifier {
    pub name: String,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NextBasicBlock {
    Jump(Entity),
    None,
}

#[derive(Debug, Copy, Clone, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Variable {
    /// The [`Identifier`] containing the variable's name.
    pub name: Entity,
    /// The variable type's [`Identifier`].
    pub ty: Entity,
}
