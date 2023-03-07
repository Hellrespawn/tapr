use crate::error::{Error, ErrorKind};
use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

pub fn tail(
    parameters: &Parameters,
    argument_nodes: &[Expression],
    intp: &mut Interpreter,
) -> Result<Value> {
    let mut arguments = parameters.evaluate_arguments(intp, argument_nodes)?;

    let Value::List(mut list) = arguments.pop().expect("tail to have one argument") else {
            unreachable!()
        };

    list.pop()
        .ok_or(Error::without_location(ErrorKind::TailOnEmptyList))
}

pub fn tail_params() -> Parameters {
    Parameters::new(vec![Parameter::new(
        "_tail".to_owned(),
        vec![ParameterType::List],
        false,
    )])
    .expect("tail to have valid params.")
}
