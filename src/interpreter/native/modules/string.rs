#![allow(clippy::unnecessary_wraps)]
use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct StringModule;

impl NativeModule for StringModule {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("len", len, "s:string"),
            ("join", join, "separator:string & s:string"),
            (
                "join-not-nil",
                join_not_nil,
                "separator:string & s:string|nil",
            ),
            ("trim", trim, "s:string"),
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

fn unary(op: UnaryOp, arguments: &Arguments) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    Ok(Value::String(op(&string)))
}

fn len(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    #[allow(clippy::cast_precision_loss)]
    Ok(Value::Number(string.len() as f64))
}

fn join(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let items = arguments.unwrap_strings_from(1);

    Ok(Value::String(items.join(&separator)))
}

fn join_not_nil(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let separator = arguments.unwrap_string(0);
    let items = arguments
        .unwrap_from(1)
        .into_iter()
        .filter_map(|v| match v {
            Value::String(s) => Some(s),
            Value::Nil => None,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    Ok(Value::String(items.join(&separator)))
}

fn trim(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    unary(|s| s.trim().to_owned(), arguments)
}
