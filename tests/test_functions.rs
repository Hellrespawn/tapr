mod common;

use common::{run_test, run_test_and_capture_output, TestResult};
use korisp::interpreter::Value;

#[test]
fn test_empty_function() -> TestResult {
    let source = "(def empty () (+ 1 2) )";

    run_test(source, Value::Nil)
}

#[test]
fn test_shadowing() -> TestResult {
    let source = include_str!("function_scope_test.ksp");

    let (_, output) = run_test_and_capture_output(source)?;

    assert_eq!("4\n40", output.trim());

    Ok(())
}
