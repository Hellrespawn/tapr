use korisp::error::ErrorKind;
use korisp::interpreter::Value;

use crate::interpreter::{expect, interpret};
use crate::TestResult;

#[test]
fn test_add() -> TestResult {
    let result = interpret("(+ 1)");

    assert!(result.is_err(), "Unary + did not return error");

    let error = result.unwrap_err();
    if !matches!(error.kind, ErrorKind::WrongAmountOfMinArgs { .. }) {
        panic!("Unary + did not return expected error, got {:?}", error)
    }

    expect("(+ 1 2)", Value::Number(3.0))?;
    expect("(+ 1 2 3 4)", Value::Number(10.0))?;

    Ok(())
}

#[test]
fn test_sub() -> TestResult {
    expect("(- 1)", Value::Number(-1.0))?;

    expect("(- 2 1)", Value::Number(1.0))?;
    expect("(- 1 2)", Value::Number(-1.0))?;
    expect("(- 3 2 1)", Value::Number(0.0))?;

    Ok(())
}

#[test]
fn test_mul() -> TestResult {
    let result = interpret("(* 1)");

    assert!(result.is_err(), "Unary * did not return error");

    let error = result.unwrap_err();
    if !matches!(error.kind, ErrorKind::WrongAmountOfMinArgs { .. }) {
        panic!("Unary * did not return expected error, got {:?}", error)
    }

    expect("(* 1 2)", Value::Number(2.0))?;
    expect("(* 1 2 3 4)", Value::Number(24.0))?;

    Ok(())
}

#[test]
fn test_div() -> TestResult {
    let result = interpret("(/ 1)");

    assert!(result.is_err(), "Unary / did not return error");

    let error = result.unwrap_err();
    if !matches!(error.kind, ErrorKind::WrongAmountOfMinArgs { .. }) {
        panic!("Unary * did not return expected error, got {:?}", error)
    }

    expect("(/ 1 2)", Value::Number(1.0 / 2.0))?;
    expect("(/ 2 1)", Value::Number(2.0 / 1.0))?;

    expect("(/ 1 2 3 4)", Value::Number(1.0 / 2.0 / 3.0 / 4.0))?;

    Ok(())
}
