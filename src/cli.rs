use std::fs;

use crate::parser::{Parser, Rule};
use pest::Parser as PestParser;

pub fn main() {
    let unparsed_file =
        fs::read_to_string("test.tsp").expect("cannot read file");

    let result = Parser::parse(Rule::main, &unparsed_file)
        .map(|mut p| p.next().unwrap());

    dbg!(result);
}
