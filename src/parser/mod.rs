#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

pub mod ast;
pub mod parameters;
