use super::expect;
use crate::TestResult;
use korisp::error::ErrorKind;
use korisp::interpreter::Value;

#[test]
fn test_can_only_read_variable_after_setting() -> TestResult {
    let error = expect("(== value 1)", Value::Boolean(true)).unwrap_err();

    assert!(matches!(error.kind, ErrorKind::UndefinedSymbol { .. }));

    expect("(tail ((def value 1) (== value 1)))", Value::Boolean(true))?;

    Ok(())
}
