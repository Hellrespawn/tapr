use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Expression;
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

pub fn head_tail_params() -> Parameters {
    Parameters::new(vec![Parameter::new(
        "_param".to_owned(),
        vec![ParameterType::List],
        false,
    )])
    .unwrap()
}
