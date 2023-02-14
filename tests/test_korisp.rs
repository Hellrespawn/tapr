use korisp::{
    interpreter::{Interpreter, Value},
    lexer::Lexer,
    parser::Parser,
    Error, Result,
};

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
    run_test("(set value 1)", Value::Nil)?;

    let error = run_test("(== value 1)", Value::Boolean(true)).unwrap_err();

    assert!(matches!(error, Error::UndefinedSymbol(_)));

    run_test("(set value 1)(== value 1)", Value::Boolean(true))?;

    Ok(())
}
