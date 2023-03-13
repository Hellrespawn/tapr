use korisp::interpreter::Value;

use crate::interpreter::capture;
use crate::TestResult;

mod arithmetic;
mod boolean;
mod list;

fn run_print_test(source: &str, expectation: &str) -> TestResult {
    let (value, string) = capture(source)?;

    assert!(matches!(value, Value::Nil));
    assert_eq!(string.trim(), expectation);

    Ok(())
}

#[test]
fn test_print() -> TestResult {
    run_print_test("(print 1)", "1")?;
    run_print_test("(print 'a)", "a")?;
    run_print_test("(print true)", "true")?;
    run_print_test("(print \"string\")", "string")?;
    run_print_test("(print (1\t2 3  4\n 5))", "(1 2 3 4 5)")?;

    Ok(())
}
