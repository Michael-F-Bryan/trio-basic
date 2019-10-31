#![feature(try_trait)]
#![deny(intra_doc_link_resolution_failure, private_in_public)]

mod machine;
mod process;
mod process_state;
mod value;

pub use crate::{
    machine::{BasicMachine, Machine},
    process::Process,
    process_state::{Continuation, Ctx, Fault, ProcessState, StackFrame},
    value::Value,
};
