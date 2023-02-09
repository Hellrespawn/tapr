#![warn(clippy::pedantic)]
#![allow(clippy::manual_assert)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]

pub mod builtin;
pub mod cli;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;
