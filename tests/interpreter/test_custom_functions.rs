use super::{capture, interpret};
use crate::TestResult;
use korisp::interpreter::Value;

#[test]
fn test_empty_function() -> TestResult {
    let source = "(lambda () (+ 1 2) )";

    let value = interpret(source)?;

    let Value::Lambda(lambda) = value else {
        panic!("Expected lambda, got '{value}'")
    };

    assert!(lambda.parameters.is_empty());

    Ok(())
}

#[test]
fn test_shadowing() -> TestResult {
    let source = include_str!("function_scope_test.ksp");

    let (_, output) = capture(source)?;

    assert_eq!("4\n40", output.trim());

    Ok(())
}
