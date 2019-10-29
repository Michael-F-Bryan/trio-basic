#![feature(try_trait)]
#![warn(intra_doc_link_resolution_failure)]

mod machine;
mod process;
mod process_state;
mod value;

pub use crate::{
    machine::{Machine, NopMachine},
    process::Process,
    process_state::{Continuation, Ctx, Fault, ProcessState, StackFrame},
    value::Value,
};
