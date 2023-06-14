use crate::interpreter::{
    Arguments, Interpreter, Parameter, ParameterType, Parameters, Value,
};
use crate::Result;

pub fn env(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = Parameters::none();
    Arguments::new(&parameters, arguments)?;

    writeln!(intp.output, "{}", intp.environment)?;

    Ok(Value::Nil)
}

pub fn lsmod(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters =
        Parameter::anonymous(vec![ParameterType::Module], false).into();
    let arguments = Arguments::new(&parameters, arguments)?;

    let (name, environment) = arguments.unwrap_module(0);

    writeln!(intp.output, "{environment}")?;

    Ok(Value::Nil)
}
