use super::{expect, get_output};
use crate::TestResult;
use korisp::interpreter::Value;

#[test]
fn test_empty_function() -> TestResult {
    let source = "(def empty () (+ 1 2) )";

    expect(source, Value::Nil)
}

#[test]
fn test_shadowing() -> TestResult {
    let source = include_str!("function_scope_test.ksp");

    let (_, output) = get_output(source)?;

    assert_eq!("4\n40", output.trim());

    Ok(())
}
