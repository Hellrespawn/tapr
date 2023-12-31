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

fn unary(op: UnaryOp, arguments: Arguments) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    Ok(Value::String(op(&string)))
}

fn len(_: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    #[allow(clippy::cast_precision_loss)]
    Ok(Value::Number(string.len() as f64))
}

fn join(_: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let values = arguments.unwrap_list(1);

    let strings = values
        .into_iter()
        .map(|value| {
            if let Value::String(s) = value {
                Ok(s)
            } else {
                Err(ErrorKind::InvalidArgument {
                    expected: vec![ParameterType::String],
                    actual: value,
                }
                .into())
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Value::String(strings.join(&separator)))
}

fn join_not_nil(_: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let values = arguments.unwrap_list(1);

    let strings = values
        .into_iter()
        .filter_map(|value| {
            match value {
                Value::String(s) => Some(Ok(s)),
                Value::Nil => None,
                other => {
                    Some(Err(ErrorKind::InvalidArgument {
                        expected: vec![ParameterType::String],
                        actual: other,
                    }
                    .into()))
                },
            }
        })
        .collect::<Result<Vec<_>>>()?;

    if strings.is_empty() {
        Ok(Value::Nil)
    } else {
        Ok(Value::String(strings.join(&separator)))
    }
}

fn trim(_: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    unary(|s| s.trim().to_owned(), arguments)
}

fn split(_: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let string = arguments.unwrap_string(1);

    let values = string
        .split(&separator)
        .map(|s| Value::String(s.to_owned()))
        .collect::<Vec<_>>();

    Ok(Value::List(values))
}
