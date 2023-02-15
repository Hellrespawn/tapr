use korisp::lexer::Lexer;
use korisp::parser::Parser;
use korisp::visitor::interpreter::{Interpreter, Value};
use korisp::{Error, Result};

type TestResult = Result<()>;

fn run_test(source: &str, expectation: Value) -> TestResult {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = parser.parse()?;
    let mut intp = Interpreter::new();

    let value = intp.interpret(&program)?;

    assert_eq!(value, expectation);

    Ok(())
}

#[test]
fn test_empty_input() -> TestResult {
    run_test("", Value::Nil)
}

#[test]
fn test_unmatched_parenthesis() -> TestResult {
    let error = run_test("(", Value::Nil).unwrap_err();

    if let Error::ConsumeError(string) = error {
        assert!(string.to_lowercase().contains("expected ')'"));
    } else {
        panic!("Expected ConsumeError, got {}", error);
    }

    Ok(())
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

#[test]
fn test_global_variables() -> TestResult {
    run_test("(var value 1)", Value::Nil)?;

    let error = run_test("(== value 1)", Value::Boolean(true)).unwrap_err();

    assert!(matches!(error, Error::UndefinedSymbol(_)));

    run_test("(var value 1)(== value 1)", Value::Boolean(true))?;

    Ok(())
}
