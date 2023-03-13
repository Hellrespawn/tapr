use korisp::interpreter::Value;

use crate::interpreter::expect;
use crate::TestResult;

fn to_val(numbers: Vec<f64>) -> Vec<Value> {
    numbers.into_iter().map(Value::Number).collect()
}

#[test]
fn test_list() -> TestResult {
    expect("(list)", Value::List(Vec::new()))?;

    expect(
        "(list 1 2 3 4)",
        Value::List(to_val(vec![1.0, 2.0, 3.0, 4.0])),
    )?;

    Ok(())
}

#[test]
fn test_head() -> TestResult {
    expect("(head ())", Value::Nil)?;

    expect("(head (1 2 3 4))", Value::Number(1.0))?;

    expect(r#"(head ("a" "b" "c"))"#, Value::String("a".to_owned()))?;

    Ok(())
}

#[test]
fn test_last() -> TestResult {
    expect("(last ())", Value::Nil)?;

    expect("(last (1 2 3 4))", Value::Number(4.0))?;

    expect(r#"(last ("a" "b" "c"))"#, Value::String("c".to_owned()))?;

    Ok(())
}

#[test]
fn test_tail() -> TestResult {
    expect("(tail ())", Value::List(Vec::new()))?;

    expect("(tail (1 2 3 4))", Value::List(to_val(vec![2.0, 3.0, 4.0])))?;

    Ok(())
}

#[test]
fn test_concat() -> TestResult {
    expect("(concat () 1)", Value::List(to_val(vec![1.0])))?;

    expect(
        "(concat (1 2 3) 4)",
        Value::List(to_val(vec![1.0, 2.0, 3.0, 4.0])),
    )?;

    let expectation = Value::List(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::List(vec![Value::Number(3.0), Value::Number(4.0)]),
    ]);

    expect("(concat (1 2) (3 4))", expectation)?;

    Ok(())
}
