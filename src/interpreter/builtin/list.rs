use crate::error::{Error, ErrorKind};
use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::{Datum, Expression};
use crate::Result;

pub fn head(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    let mut arguments =
        head_tail_params().evaluate_arguments(intp, argument_nodes)?;

    let Value::List(list) = arguments.pop().expect("head to have one argument") else {
            unreachable!("Should be checked by params.")
        };

    Ok(list.into_iter().next().unwrap_or_else(|| Value::Nil))
}

pub fn tail(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    let mut arguments =
        head_tail_params().evaluate_arguments(intp, argument_nodes)?;

    let Value::List(mut list) = arguments.pop().expect("head to have one argument") else {
            unreachable!("Should be checked by params.")
        };

    Ok(list.pop().unwrap_or_else(|| Value::Nil))
}

pub fn map(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    map_params().check_amount_of_args_or_error(argument_nodes.len())?;

    let function = argument_nodes
        .get(0)
        .expect("map should be called with a function.")
        .accept(intp)?;

    if !matches!(function, Value::Builtin(_) | Value::Lambda(_)) {
        return Err(Error::without_location(ErrorKind::InvalidArgument {
            expected: vec![ParameterType::Function],
            actual: function,
        }));
    }

    let expression = argument_nodes
        .get(1)
        .expect("map should be called with a list.");

    let Expression::Datum(Datum::List(list)) = expression else {
        return Err(Error::without_location(ErrorKind::InvalidArgument {
            expected: vec![ParameterType::List],
            actual: expression.accept(intp)?,
        }));
    };

    let mut out = Vec::new();

    for i in 0..list.expressions.len() {
        out.push(match &function {
            Value::Builtin(builtin) => {
                builtin.call(intp, &list.expressions[i..=i])?
            }
            Value::Lambda(lambda) => {
                lambda.call(intp, &list.expressions[i..=i])?
            }
            _ => unreachable!("checked above."),
        });
    }

    Ok(Value::List(out))
}

pub fn head_tail_params() -> Parameters {
    Parameters::new(vec![Parameter::new(
        "_param".to_owned(),
        vec![ParameterType::List],
        false,
    )])
    .unwrap()
}

pub fn map_params() -> Parameters {
    Parameters::new(vec![
        Parameter::new(
            "_function".to_owned(),
            vec![ParameterType::Function],
            false,
        ),
        Parameter::new("_list".to_owned(), vec![ParameterType::List], false),
    ])
    .unwrap()
}
