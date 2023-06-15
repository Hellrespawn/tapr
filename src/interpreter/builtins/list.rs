use crate::interpreter::parameters::{Parameter, Parameters};
use crate::interpreter::value::Callable;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

#[allow(clippy::unnecessary_wraps)]
pub fn list(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    Ok(Value::List(arguments))
}

pub fn head(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = Parameter::new("list".to_owned()).list().into();

    let arguments = Arguments::new(&parameters, arguments)?;

    let list = arguments.unwrap_list(0);

    Ok(list.into_iter().next().unwrap_or_else(|| Value::Nil))
}

pub fn tail(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = Parameter::new("list".to_owned()).list().into();

    let arguments = Arguments::new(&parameters, arguments)?;

    let list = arguments
        .unwrap_list(0)
        .get(1..)
        .map(Vec::from)
        .unwrap_or_default();

    Ok(Value::List(list))
}

pub fn peek(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = Parameter::new("list".to_owned()).list().into();

    let arguments = Arguments::new(&parameters, arguments)?;

    let mut list = arguments.unwrap_list(0);

    Ok(list.pop().unwrap_or_else(|| Value::Nil))
}

pub fn push(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = push_params();

    let arguments = Arguments::new(&parameters, arguments)?;

    let list = arguments.unwrap_list(0);

    let values = arguments.arguments()[1..].to_owned();

    let output = [list, values].into_iter().flatten().collect();

    Ok(Value::List(output))
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
            Value::Function(function) => function.call(intp, vec![value]),
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
            Value::Function(function) => {
                function.call(intp, vec![value.clone()])
            }
            _ => unreachable!("checked above."),
        }?
        .is_truthy();

        if is_truthy {
            output.push(value);
        }
    }

    Ok(Value::List(output))
}

pub fn reduce(intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    let parameters = reduce_params();
    let arguments = Arguments::new(&parameters, arguments)?;

    let init = arguments.unwrap(0);
    let function = arguments.unwrap_function(1);
    let input = arguments.unwrap_list(2);

    let mut output = init;

    for value in input {
        output = match &function {
            Value::Builtin(builtin) => builtin.call(intp, vec![output, value]),
            Value::Function(function) => {
                function.call(intp, vec![output, value])
            }
            _ => unreachable!("checked above."),
        }?;
    }

    Ok(output)
}

pub fn push_params() -> Parameters {
    Parameters::new(vec![
        Parameter::new("list".to_owned()).list(),
        Parameter::new("element".to_owned()).rest(),
    ])
    .unwrap()
}

pub fn map_filter_params() -> Parameters {
    Parameters::new(vec![
        Parameter::new("function".to_owned()).function(),
        Parameter::new("list".to_owned()).list(),
    ])
    .unwrap()
}

pub fn reduce_params() -> Parameters {
    Parameters::new(vec![
        Parameter::new("initial".to_owned()),
        Parameter::new("function".to_owned()).function(),
        Parameter::new("list".to_owned()).list(),
    ])
    .unwrap()
}
