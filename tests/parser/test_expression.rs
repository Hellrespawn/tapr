use korisp::parser::ast::{Datum, Expression};

use crate::TestResult;

use super::parse;

#[test]
fn test_expression_is_list() -> TestResult {
    let source = "(1 2 3 4)";

    let expression = parse(source)?;

    let Expression::Datum(Datum::List(_)) = expression else {
        panic!("'{source}' did not return List, but {:?}", expression)
    };

    Ok(())
}

#[test]
fn test_expression_is_function_call() -> TestResult {
    let source = "(symbol 2 3 4)";

    let expression = parse(source)?;

    let Expression::Call(_) = expression else {
        panic!("'{source}' did not return FunctionCall, but {:?}", expression)
    };

    Ok(())
}

#[test]
fn test_expression_is_atom_number() -> TestResult {
    let source = "1";

    let expression = parse(source)?;

    let Expression::Datum(Datum::Number(_)) = expression else {
        panic!("'{source}' did not return Atom::Number, but {:?}", expression)
    };

    Ok(())
}

#[test]
fn test_expression_is_atom_string() -> TestResult {
    let source = "\"string\"";

    let node = parse(source)?;

    let Expression::Datum(Datum::String(_)) = node else {
        panic!("'{source}' did not return Atom::String, but {:?}", node)
    };

    Ok(())
}

#[test]
fn test_expression_is_atom_symbol() -> TestResult {
    let source = "symbol";

    let expression = parse(source)?;

    let Expression::Datum(Datum::Symbol(_)) = expression else {
        panic!("'{source}' did not return Atom::Symbol, but {:?}", expression)
    };

    Ok(())
}
