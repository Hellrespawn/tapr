mod common;

use common::{run_test, TestResult};
use korisp::error::ErrorKind;
use korisp::interpreter::Value;

#[test]
fn test_scope() -> TestResult {
    let source = include_str!("variable_scope_test.ksp");

    let error = run_test(source, Value::Number(10.0)).unwrap_err();

    assert!(matches!(error.kind, ErrorKind::UndefinedSymbol { .. }));

    Ok(())
}

#[test]
fn test_can_only_read_variable_after_setting() -> TestResult {
    let error = run_test("(== value 1)", Value::Boolean(true)).unwrap_err();

    assert!(matches!(error.kind, ErrorKind::UndefinedSymbol { .. }));

    run_test("(set (value 1) (== value 1))", Value::Boolean(true))?;

    Ok(())
}
