use super::expect;
use crate::TestResult;
use korisp::error::ErrorKind;
use korisp::interpreter::Value;

#[test]
fn test_scope() -> TestResult {
    let source = include_str!("variable_scope_test.ksp");

    let error = expect(source, Value::Number(10.0)).unwrap_err();

    assert!(matches!(error.kind, ErrorKind::UndefinedSymbol { .. }));

    Ok(())
}

#[test]
fn test_can_only_read_variable_after_setting() -> TestResult {
    let error = expect("(== value 1)", Value::Boolean(true)).unwrap_err();

    assert!(matches!(error.kind, ErrorKind::UndefinedSymbol { .. }));

    expect("(set (value 1) (== value 1))", Value::Boolean(true))?;

    Ok(())
}
