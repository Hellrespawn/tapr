#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::location::Location;
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

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "list"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn head(
    _location: Location,
    _: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let list = arguments.unwrap_list(0);

    Ok(list.into_iter().next().unwrap_or_else(Value::nil))
}

fn tail(
    _location: Location,
    _: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let list = arguments
        .unwrap_list(0)
        .get(1..)
        .map(Vec::from)
        .unwrap_or_default();

    Ok(Value::b_tuple(list))
}

fn push(
    _location: Location,
    _: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let list = arguments.unwrap_list(0);

    let values = arguments.arguments()[1..].to_owned();

    let output = [list, values].into_iter().flatten().collect();

    Ok(Value::b_tuple(output))
}

fn reduce(
    location: Location,
    intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let function = arguments.unwrap_function(0);
    let init = arguments.unwrap(1);
    let input = arguments.unwrap_list(2);

    let mut output = init;

    for value in input {
        output = function.call(
            location,
            intp,
            Arguments::from_values(
                &function.parameters(),
                vec![output, value],
            )?,
        )?;
    }

    Ok(output)
}

fn filter(
    location: Location,
    intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let function = arguments.unwrap_function(0);
    let values = arguments.unwrap_list(1);

    let mut output = Vec::new();

    for value in values {
        let is_truthy = function
            .call(
                location,
                intp,
                Arguments::from_values(
                    &function.parameters(),
                    vec![value.clone()],
                )?,
            )?
            .is_truthy();

        if is_truthy {
            output.push(value);
        }
    }

    Ok(Value::b_tuple(output))
}

fn map(
    location: Location,
    intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let function = arguments.unwrap_function(0);
    let values = arguments.unwrap_list(1);

    let output = values
        .into_iter()
        .map(|v| {
            function.call(
                location,
                intp,
                Arguments::from_values(&function.parameters(), vec![v])?,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Value::b_tuple(output))
}
