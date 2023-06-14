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
mod env;
mod error;
mod graph;
mod interpreter;
mod location;
mod parser;
mod visitor;

pub type Result<T> = std::result::Result<T, error::Error>;

// TODO More sophisticated command line handling.
// TODO Save repl-history in a config dir
// TODO Allow resetting of REPL history
