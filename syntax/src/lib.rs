//! The frontend code in charge of turning a stream of characters into a full
//! Abstract Syntax Tree.

extern crate heapsize;
#[macro_use]
extern crate heapsize_derive;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate codespan;
extern crate failure;
extern crate lalrpop_util;
extern crate regex;
#[macro_use]
extern crate failure_derive;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
mod macros;
pub mod ast;
mod errors;
pub mod lexer;
pub mod tokens;

pub use errors::ParseError;

lalrpop_util::lalrpop_mod!(pub(crate) grammar);

use ast::{AstNode, File};
use codespan::{ByteOffset, FileMap};

/// Parse the contents of a `codespan::FileMap` and automatically update the
/// AST's spans appropriately.
pub fn parse_from_filemap(filemap: &FileMap) -> Result<File, ParseError> {
    let offset = ByteOffset(filemap.span().start().0 as i64);

    let mut ast = parse(filemap.src()).map_err(|mut e| {
        e.offset_inplace(offset);
        e
    })?;

    ast.offset_inplace(offset);

    debug_assert!(
        filemap.span().contains(ast.span()),
        "The AST's span lies outside of the filemap ({:?} is not in {:?}). This is a bug.",
        ast.span(),
        filemap.span()
    );

    Ok(ast)
}

/// Parse a raw source string into its AST representation.
///
/// # Note
///
/// Spans and location info is relative to the start of the string. If this is
/// important to you or you want to get useful error messages (e.g. error
/// messages that actually point to where the error happened) you should either
/// update locations via the `offset_inplace()` method or use the
/// `parse_from_filemap()` map.
pub fn parse(src: &str) -> Result<File, ParseError> { File::from_str(src) }
