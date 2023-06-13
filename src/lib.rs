#![warn(clippy::pedantic)]
#![allow(clippy::manual_assert)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
#![allow(clippy::wildcard_imports)]

extern crate alloc;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod cli;
mod error;
mod graph;
mod interpreter;
mod location;
mod parser;
mod visitor;

pub type Result<T> = std::result::Result<T, error::Error>;

// TODO? Add module system
// (mod "filename")
// Runs filename in it's own scope, then makes the top-level environment
// available in a new Value::Module(env).
// (mod/val) attempts to retrieve val from mod.
// Functions from module must run in the module's env
