use crate::interpreter::parameters::{Parameter, ParameterType};
use crate::interpreter::{Arguments, Interpreter, Parameters, Value};
use crate::Result;

pub fn read(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params =
        Parameter::anonymous(vec![ParameterType::String], false).into();

    let arguments = Arguments::new(&params, arguments)?;

    let path = arguments.unwrap_string(0);

    let string = std::fs::read_to_string(path)?;

    Ok(Value::String(string))
}

pub fn write(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = Parameters::new(vec![
        Parameter::new("path".to_owned(), vec![ParameterType::String], false),
        Parameter::new("body".to_owned(), vec![ParameterType::String], false),
    ])?;

    let arguments = Arguments::new(&params, arguments)?;

    let path = arguments.unwrap_string(0);
    let body = arguments.unwrap_string(1);

    std::fs::write(path, body)?;

    Ok(Value::Nil)
}
