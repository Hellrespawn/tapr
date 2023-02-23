use korisp::parser::ast::{Atom, Node};

use crate::TestResult;

use super::parse;

#[test]
fn test_expression_is_list() -> TestResult {
    let source = "(1 2 3 4)";

    let node = parse(source)?;

    let Node::List(_) = node else {
        panic!("'{source}' did not return List, but {}", node.node_type())
    };

    Ok(())
}

#[test]
fn test_expression_is_function_call() -> TestResult {
    let source = "(symbol 2 3 4)";

    let node = parse(source)?;

    let Node::FunctionCall(_) = node else {
        panic!("'{source}' did not return FunctionCall, but {}", node.node_type())
    };

    Ok(())
}

#[test]
fn test_expression_is_atom_number() -> TestResult {
    let source = "1";

    let node = parse(source)?;

    let Node::Atom(Atom::Number(_)) = node else {
        panic!("'{source}' did not return Atom::Number, but {}", node.node_type())
    };

    Ok(())
}

#[test]
fn test_expression_is_atom_string() -> TestResult {
    let source = "\"string\"";

    let node = parse(source)?;

    let Node::Atom(Atom::String(_)) = node else {
        panic!("'{source}' did not return Atom::String, but {}", node.node_type())
    };

    Ok(())
}

#[test]
fn test_expression_is_atom_symbol() -> TestResult {
    let source = "symbol";

    let node = parse(source)?;

    let Node::Atom(Atom::Symbol(_)) = node else {
        panic!("'{source}' did not return Atom::Symbol, but {}", node.node_type())
    };

    Ok(())
}
