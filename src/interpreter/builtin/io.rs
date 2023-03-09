use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub fn print(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let params = print_params();

    let arguments = Arguments::new(&params, arguments)?;

    for argument in arguments.arguments() {
        writeln!(intp.output, "{argument}")?;
    }

    Ok(Value::Nil)
}

pub fn print_params() -> Parameters {
    Parameters::new(vec![Parameter::anonymous(ParameterType::Any, true)])
        .expect("print to have valid params.")
}
