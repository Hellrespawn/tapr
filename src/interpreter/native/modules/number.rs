#![allow(clippy::unnecessary_wraps)]
use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::error::ErrorKind;
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct Number;

impl NativeModule for Number {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> =
            vec![("parse", parse, "s:string")];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "number"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

pub fn parse(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    let result: Result<f64> = string
        .trim()
        .parse()
        .map_err(|_| ErrorKind::ParseNumberError(string).into());

    Ok(Value::Number(result?))
}
