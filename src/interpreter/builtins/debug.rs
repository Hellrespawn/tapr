use crate::interpreter::{Arguments, Interpreter, Parameters, Value};
use crate::Result;

pub fn env(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = Parameters::none();
    Arguments::new(&parameters, arguments)?;

    writeln!(intp.output, "{}", intp.environment)?;

    Ok(Value::Nil)
}
