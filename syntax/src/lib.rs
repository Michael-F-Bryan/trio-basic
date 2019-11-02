//! The frontend code in charge of turning a stream of characters into a full
//! Abstract Syntax Tree.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[macro_use]
extern crate lalrpop_util;

#[macro_use]
mod macros;
pub mod ast;
mod errors;
pub mod lexer;
pub mod tokens;

pub use crate::errors::ParseError;

lalrpop_mod!(pub(crate) grammar);

use crate::ast::File;

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
