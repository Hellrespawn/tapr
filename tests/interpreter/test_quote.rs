use super::capture;
use crate::TestResult;
use korisp::interpreter::Value;

fn construct_source(source: &str) -> String {
    format!(
        "(tail
            (
                (def add-one (lambda (a) (+ 1 a)))
                {}
            )
        )",
        source
    )
    .trim()
    .to_owned()
}

#[test]
fn test_no_quote() -> TestResult {
    let (value, _) = capture(&construct_source("(add-one 2)"))?;

    let Value::Number(number) = value else {
        panic!("Expected number, got '{value}'")
    };

    assert!((number - 3.0).abs() < f64::EPSILON);

    Ok(())
}

#[test]
fn test_quote_symbol() -> TestResult {
    let (value, _) = capture(&construct_source("('add-one 2)"))?;

    let Value::List(list) = value else {
        panic!("Expected list, got '{value}'")
    };

    assert!(list.len() == 2, "Got more than two elements.");

    let Value::Symbol(symbol) = &list[0] else {
        panic!("Expected symbol as first element, got '{}'", &list[0])
    };

    assert_eq!(symbol, "add-one");

    let Value::Number(number) = &list[1] else {
        panic!("Expected number as second element, got '{}'", &list[1])
    };

    assert!((number - 2.0).abs() < f64::EPSILON);

    Ok(())
}

#[test]
fn test_quote_list() -> TestResult {
    let (value, _) = capture(&construct_source("'(add-one 2)"))?;

    let Value::List(list) = value else {
        panic!("Expected list, got '{value}'")
    };

    assert!(list.len() == 2, "Got more than two elements.");

    let Value::Lambda(_) = &list[0] else {
        panic!("Expected lambda as first element, got '{}'", &list[0])
    };

    let Value::Number(number) = &list[1] else {
        panic!("Expected number as second element, got '{}'", &list[1])
    };

    assert!((number - 2.0).abs() < f64::EPSILON);

    Ok(())
}

#[test]
fn test_quote_both() -> TestResult {
    let (value, _) = capture(&construct_source("('add-one 2)"))?;

    let Value::List(list) = value else {
        panic!("Expected list, got '{value}'")
    };

    assert!(list.len() == 2, "Got more than two elements.");

    let Value::Symbol(symbol) = &list[0] else {
        panic!("Expected symbol as first element, got '{}'", &list[0])
    };

    assert_eq!(symbol, "add-one");

    let Value::Number(number) = &list[1] else {
        panic!("Expected number as second element, got '{}'", &list[1])
    };

    assert!((number - 2.0).abs() < f64::EPSILON);

    Ok(())
}
