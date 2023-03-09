use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub fn head(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = head_tail_params();

    let arguments = Arguments::new(&parameters, arguments)?;

    let list = arguments.unwrap_list(0);

    Ok(list.into_iter().next().unwrap_or_else(|| Value::Nil))
}

pub fn tail(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = head_tail_params();

    let arguments = Arguments::new(&parameters, arguments)?;

    let mut list = arguments.unwrap_list(0);

    Ok(list.pop().unwrap_or_else(|| Value::Nil))
}

pub fn map(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = map_filter_params();

    let arguments = Arguments::new(&parameters, arguments)?;

    let function = arguments.unwrap_function(0);

    let input = arguments.unwrap_list(1);

    let output = input
        .into_iter()
        .map(|value| match &function {
            Value::Builtin(builtin) => builtin.call(intp, vec![value]),
            Value::Lambda(lambda) => lambda.call(intp, vec![value]),
            _ => unreachable!("checked above."),
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Value::List(output))
}

pub fn filter(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = map_filter_params();

    let arguments = Arguments::new(&parameters, arguments)?;

    let function = arguments.unwrap_function(0);

    let input = arguments.unwrap_list(1);

    let mut output = Vec::new();

    for value in input {
        let is_truthy = match &function {
            Value::Builtin(builtin) => builtin.call(intp, vec![value.clone()]),
            Value::Lambda(lambda) => lambda.call(intp, vec![value.clone()]),
            _ => unreachable!("checked above."),
        }?
        .is_truthy();

        if is_truthy {
            output.push(value);
        }
    }

    Ok(Value::List(output))
}

pub fn head_tail_params() -> Parameters {
    Parameters::new(vec![Parameter::anonymous(ParameterType::List, false)])
        .unwrap()
}

pub fn map_filter_params() -> Parameters {
    Parameters::new(vec![
        Parameter::anonymous(ParameterType::Function, false),
        Parameter::anonymous(ParameterType::List, false),
    ])
    .unwrap()
}
