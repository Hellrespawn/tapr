use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

pub fn print(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    let arguments = print_params().evaluate_arguments(intp, argument_nodes)?;

    for argument in arguments {
        writeln!(intp.output, "{argument}")?;
    }

    Ok(Value::Nil)
}

pub fn print_params() -> Parameters {
    Parameters::new(vec![Parameter::new(
        "_print".to_owned(),
        vec![ParameterType::Any],
        true,
    )])
    .expect("print to have valid params.")
}
