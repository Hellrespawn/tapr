use korisp::interpreter::{Interpreter, Value};
use korisp::Result;

pub type TestResult = Result<()>;
pub type CapturedTestResult = Result<(Value, String)>;

pub fn run_test(source: &str, expectation: Value) -> TestResult {
    let mut intp = Interpreter::default();
    intp.output = Box::new(std::io::sink());

    let value = intp.interpret(source)?;

    assert_eq!(value, expectation);

    Ok(())
}

pub fn run_test_and_capture_output(source: &str) -> CapturedTestResult {
    let mut buffer = Vec::new();

    let mut intp = Interpreter::default();
    intp.output = Box::new(&mut buffer);

    let value = intp.interpret(source)?;

    std::mem::drop(intp);

    let output =
        std::str::from_utf8(&buffer).expect("Output to be valid UTF-8");

    Ok((value, output.to_owned()))
}
