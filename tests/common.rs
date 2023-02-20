use korisp::lexer::Lexer;
use korisp::parser::Parser;
use korisp::visitor::interpreter::{Interpreter, Value};
use korisp::Result;

pub type TestResult = Result<()>;

pub fn run_test(source: &str, expectation: Value) -> TestResult {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let program = parser.parse()?;
    let mut intp = Interpreter::new();

    let value = intp.interpret(&program)?;

    assert_eq!(value, expectation);

    Ok(())
}
