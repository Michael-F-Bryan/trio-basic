#![warn(intra_doc_link_resolution_failure)]

mod machine;
mod process;
mod value;

pub use crate::machine::{Machine, NopMachine};
pub use crate::process::{Builder, Process};
pub use crate::value::Value;
