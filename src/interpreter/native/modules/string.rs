#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::error::ErrorKind;
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::{ParameterType, Result};

pub struct StringModule;

impl NativeModule for StringModule {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("len", len, "s:string"),
            ("join", join, "separator:string l:list"),
            ("join-not-nil", join_not_nil, "separator:string l:list"),
            ("trim", trim, "s:string"),
            ("split", split, "separator:string string:string"),
        ];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "string"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

type UnaryOp = fn(&str) -> String;

fn unary(op: UnaryOp, arguments: Arguments<Value>) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    Ok(Value::string(op(&string)))
}

fn len(_: &mut Interpreter, arguments: Arguments<Value>) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    #[allow(clippy::cast_precision_loss)]
    Ok(Value::number(string.len() as f64))
}

fn join(_: &mut Interpreter, arguments: Arguments<Value>) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let values = arguments.unwrap_list(1);

    let strings = values
        .into_iter()
        .map(|value| {
            if let Some(string) = value.as_string() {
                Ok(string.to_owned())
            } else {
                Err(ErrorKind::InvalidValueArgument {
                    expected: vec![ParameterType::String],
                    actual: value,
                }
                .into())
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Value::string(strings.join(&separator)))
}

fn join_not_nil(
    _: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let values = arguments.unwrap_list(1);

    let strings = values
        .into_iter()
        .filter_map(|value| {
            if value.is_nil() {
                None
            } else if let Some(string) = value.as_string() {
                Some(Ok(string.to_owned()))
            } else {
                Some(Err(ErrorKind::InvalidValueArgument {
                    expected: vec![ParameterType::String],
                    actual: value,
                }
                .into()))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    if strings.is_empty() {
        Ok(Value::nil())
    } else {
        Ok(Value::string(strings.join(&separator)))
    }
}

fn trim(_: &mut Interpreter, arguments: Arguments<Value>) -> Result<Value> {
    unary(|s| s.trim().to_owned(), arguments)
}

fn split(_: &mut Interpreter, arguments: Arguments<Value>) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let string = arguments.unwrap_string(1);

    let values = string
        .split(&separator)
        .map(|s| Value::string(s.to_owned()))
        .collect::<Vec<_>>();

    Ok(Value::b_tuple(values))
}
