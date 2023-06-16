use crate::interpreter::{Arguments, Interpreter, Value};
use crate::parser::parameters::{Parameter, Parameters};
use crate::Result;

pub fn env(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = Parameters::none();
    Arguments::new(&parameters, arguments)?;

    writeln!(intp.output, "{}", intp.environment)?;

    Ok(Value::Nil)
}

pub fn lsmod(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = Parameter::empty("module".to_owned()).module().into();
    let arguments = Arguments::new(&parameters, arguments)?;

    let environment = arguments.unwrap_module(0);

    writeln!(intp.output, "{environment}")?;

    Ok(Value::Nil)
}
