use crate::interpreter::parameters::{Parameter, ParameterType};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub fn print(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = Parameter::anonymous(ParameterType::Any, true).try_into()?;

    let arguments = Arguments::new(&params, arguments)?;

    for argument in arguments.arguments() {
        writeln!(intp.output, "{argument}")?;
    }

    Ok(Value::Nil)
}

pub fn read(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params =
        Parameter::anonymous(ParameterType::String, false).try_into()?;

    let arguments = Arguments::new(&params, arguments)?;

    let path = arguments.unwrap_string(0);

    let string = std::fs::read_to_string(path)?;

    Ok(Value::String(string))
}

pub fn eval(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params =
        Parameter::anonymous(ParameterType::String, false).try_into()?;

    let arguments = Arguments::new(&params, arguments)?;

    let string = arguments.unwrap_string(0);

    let value = intp.interpret(&string)?;

    Ok(value)
}
