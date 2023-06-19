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

mod cli;
mod env;
mod error;
mod graph;
mod interpreter;
mod location;
mod parser;
mod visitor;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use crate::Result as TaprResult;
pub use cli::main;
pub use error::{Error as TaprError, ErrorKind as TaprErrorKind};
pub use interpreter::{
    Arguments, Callable, CallableType, Environment, Interpreter,
    NativeFunction, NativeFunctionImpl, Value,
};
pub use parser::ast::{Node, NodeData};
pub use parser::parameters::{Parameter, ParameterType, Parameters};
pub use visitor::Visitor;
