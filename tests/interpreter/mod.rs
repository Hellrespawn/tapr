mod test_builtin_functions;
mod test_custom_functions;
mod test_korisp;
mod test_variables;

use korisp::interpreter::{Interpreter, Value};
use korisp::Result;

use crate::TestResult;
pub type CapturedTestResult = Result<(Value, String)>;

pub fn expect(source: &str, expectation: Value) -> TestResult {
    let mut intp = Interpreter::default();
    intp.output = Box::new(std::io::sink());

    let value = intp.interpret(source)?;

    assert_eq!(value, expectation);

    Ok(())
}

pub fn capture(source: &str) -> CapturedTestResult {
    let mut buffer = Vec::new();

    let mut intp = Interpreter::default();
    intp.output = Box::new(&mut buffer);

    let value = intp.interpret(source)?;

    std::mem::drop(intp);

    let output =
        std::str::from_utf8(&buffer).expect("Output to be valid UTF-8");

    Ok((value, output.to_owned()))
}
