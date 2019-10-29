#[allow(unused_imports)]
use crate::Process;

/// The interface a [`Process`] can use to interact with the outside world
/// and other processes.
pub trait Machine {}

pub struct NopMachine;

impl Machine for NopMachine {}
