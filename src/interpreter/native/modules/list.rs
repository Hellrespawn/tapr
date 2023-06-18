#![allow(clippy::unnecessary_wraps)]
use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct List;

impl NativeModule for List {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("head", head, "l:list"),
            ("tail", tail, "l:list"),
            ("push", push, "l:list"),
            ("filter", filter, "f:function l:list"),
            ("map", map, "f:function l:list"),
            ("reduce", reduce, "f:function init l:list"),
        ];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "list"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn head(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let list = arguments.unwrap_list(0);

    Ok(list.into_iter().next().unwrap_or_else(|| Value::Nil))
}

fn tail(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let list = arguments
        .unwrap_list(0)
        .get(1..)
        .map(Vec::from)
        .unwrap_or_default();

    Ok(Value::List(list))
}

fn push(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let list = arguments.unwrap_list(0);

    let values = arguments.arguments()[1..].to_owned();

    let output = [list, values].into_iter().flatten().collect();

    Ok(Value::List(output))
}

fn reduce(intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let function = arguments.unwrap_function(0);
    let init = arguments.unwrap(1);
    let input = arguments.unwrap_list(2);

    let callable = function.as_callable().expect("Checked by Arguments.");

    let mut output = init;

    for value in input {
        output = callable.call(intp, vec![output, value])?;
    }

    Ok(output)
}

fn filter(intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let function = arguments.unwrap_function(0);
    let values = arguments.unwrap_list(1);

    let callable = function.as_callable().expect("Checked by Arguments.");

    let mut output = Vec::new();

    for value in values {
        let is_truthy = callable.call(intp, vec![value.clone()])?.is_truthy();

        if is_truthy {
            output.push(value);
        }
    }

    Ok(Value::List(output))
}

fn map(intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let function = arguments.unwrap_function(0);
    let values = arguments.unwrap_list(1);

    let callable = function.as_callable().expect("Checked by Arguments.");

    let output = values
        .into_iter()
        .map(|v| callable.call(intp, vec![v]))
        .collect::<Result<Vec<_>>>()?;

    Ok(Value::List(output))
}
