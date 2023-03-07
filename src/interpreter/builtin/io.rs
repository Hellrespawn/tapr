use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::{Datum, Expression};
use crate::Result;

pub fn print(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    let arguments = parameters.evaluate_arguments(intp, argument_nodes)?;

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

pub fn quote(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    parameters.check_amount_of_args_or_error(argument_nodes.len())?;

    let argument_node = argument_nodes.first().expect("one argument node");

    let value = match argument_node {
        Expression::Datum(Datum::List(list)) => {
            let elements = list
                .iter()
                .map(|node| node.accept(intp))
                .collect::<Result<Vec<_>>>()?;

            Value::List(elements)
        }
        Expression::Datum(Datum::Symbol(symbol)) => {
            Value::Symbol(symbol.lexeme().to_owned())
        }
        node => node.accept(intp)?,
    };

    Ok(value)
}

pub fn quote_params() -> Parameters {
    Parameters::new(vec![Parameter::new(
        "_quote".to_owned(),
        vec![ParameterType::Any],
        false,
    )])
    .expect("quote to have valid params.")
}
