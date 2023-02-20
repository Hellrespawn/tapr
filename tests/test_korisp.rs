mod common;

use common::{run_test, TestResult};
use korisp::error::ErrorKind;
use korisp::visitor::interpreter::Value;

#[test]
fn test_empty_input() -> TestResult {
    let error = run_test("", Value::Nil).unwrap_err();

    assert!(matches!(error.kind, ErrorKind::EmptyInput));

    Ok(())
}

#[test]
fn test_unmatched_parenthesis() -> TestResult {
    let error = run_test("(", Value::Nil).unwrap_err();

    if let ErrorKind::UnmatchedParenthesis { .. } = error.kind {
        Ok(())
    } else {
        panic!("Expected Error::UnmatchedParenthesis, got {}", error);
    }
}

#[test]
fn test_truthiness() -> TestResult {
    let values = [
        ("nil", false),
        ("true", true),
        ("false", false),
        ("0", false),
        ("0.0", false),
        ("1", true),
        ("1.0", true),
        ("\"\"", false),
        ("\"not empty\"", true),
        ("(1 2 3)", true),
    ];

    for (value, expectation) in values {
        run_test(
            &format!("(if {value} true false)"),
            Value::Boolean(expectation),
        )?;
    }

    Ok(())
}
