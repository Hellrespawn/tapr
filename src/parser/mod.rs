use once_cell::sync::Lazy;

pub static DEBUG_AST: Lazy<bool> =
    Lazy::new(|| std::env::var("DEBUG_AST").is_ok());

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

pub mod ast;
