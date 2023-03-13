use crate::interpreter::expect;
use crate::TestResult;
use korisp::interpreter::Value;

#[test]
fn test_eq() -> TestResult {
    expect("(== 1 1)", Value::Boolean(true))?;
    expect("(== 1 1 1)", Value::Boolean(true))?;

    expect("(== 1 2 3)", Value::Boolean(false))?;
    expect("(== 1 1 2 1)", Value::Boolean(false))?;

    Ok(())
}

#[test]
fn test_gte() -> TestResult {
    expect("(>= 1 2)", Value::Boolean(true))?;
    expect("(>= 1 2 3)", Value::Boolean(true))?;

    expect("(>= 2 1)", Value::Boolean(false))?;
    expect("(>= 1 2 4 3)", Value::Boolean(false))?;

    expect("(>= 1 1)", Value::Boolean(true))?;
    expect("(>= 1 1 1)", Value::Boolean(true))?;

    expect("(>= 1 1 2 1)", Value::Boolean(false))?;

    Ok(())
}

#[test]
fn test_gt() -> TestResult {
    expect("(> 1 2)", Value::Boolean(true))?;
    expect("(> 1 2 3)", Value::Boolean(true))?;

    expect("(> 2 1)", Value::Boolean(false))?;
    expect("(> 1 2 4 3)", Value::Boolean(false))?;

    Ok(())
}

#[test]
fn test_lte() -> TestResult {
    expect("(< 1 2)", Value::Boolean(false))?;
    expect("(< 1 2 3)", Value::Boolean(false))?;

    expect("(< 2 1)", Value::Boolean(true))?;
    expect("(< 4 3 2 1)", Value::Boolean(true))?;

    expect("(< 1 2 4 3)", Value::Boolean(false))?;

    expect("(<= 1 1)", Value::Boolean(true))?;
    expect("(<= 1 1 1)", Value::Boolean(true))?;

    expect("(<= 2 2 1 2)", Value::Boolean(false))?;

    Ok(())
}

#[test]
fn test_lt() -> TestResult {
    expect("(< 1 2)", Value::Boolean(false))?;
    expect("(< 1 2 3)", Value::Boolean(false))?;

    expect("(< 2 1)", Value::Boolean(true))?;
    expect("(< 4 3 2 1)", Value::Boolean(true))?;

    expect("(< 1 2 4 3)", Value::Boolean(false))?;

    Ok(())
}
