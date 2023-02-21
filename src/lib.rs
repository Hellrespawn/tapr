#![warn(clippy::pedantic)]
#![allow(clippy::manual_assert)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
#![allow(clippy::wildcard_imports)]

pub mod cli;
pub mod error;
pub mod graph;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod visitor;

pub type Result<T> = std::result::Result<T, error::Error>;
