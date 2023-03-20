use crate::error::ErrorKind;
use crate::interpreter::parameters::{Parameter, ParameterType};
use crate::interpreter::{Arguments, Interpreter, Parameters, Value};
use crate::Result;

pub fn print(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = Parameter::anonymous(ParameterType::Any, true).into();

    let arguments = Arguments::new(&params, arguments)?;

    for argument in arguments.arguments() {
        write!(intp.output, "{argument}")?;
    }

    intp.output.flush()?;

    Ok(Value::Nil)
}

pub fn println(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = Parameter::anonymous(ParameterType::Any, true).into();

    let arguments = Arguments::new(&params, arguments)?;

    for argument in arguments.arguments() {
        write!(intp.output, "{argument}")?;
    }

    writeln!(intp.output)?;

    Ok(Value::Nil)
}

pub fn read(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = Parameters::none();
    Arguments::new(&params, arguments)?;

    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;

    Ok(Value::String(buffer))
}

pub fn read_file(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    let params = Parameter::anonymous(ParameterType::String, false).into();

    let arguments = Arguments::new(&params, arguments)?;

    let path = arguments.unwrap_string(0);

    let string = std::fs::read_to_string(path)?;

    Ok(Value::String(string))
}

pub fn eval(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = Parameter::anonymous(ParameterType::String, false).into();

    let arguments = Arguments::new(&params, arguments)?;

    let string = arguments.unwrap_string(0);

    let value = intp.interpret(&string, "eval")?;

    Ok(value)
}

pub fn parse_number(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    let params = Parameter::anonymous(ParameterType::String, false).into();

    let arguments = Arguments::new(&params, arguments)?;

    let string = arguments.unwrap_string(0);

    let result: Result<f64> = string
        .trim()
        .parse()
        .map_err(|_| ErrorKind::ParseNumberError(string).into());

    Ok(Value::Number(result?))
}
