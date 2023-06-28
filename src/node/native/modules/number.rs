#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::error::ErrorKind;
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;
use conv::prelude::*;

pub struct Number;

impl NativeModule for Number {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("parse", parse, "s:string"),
            ("align", align, "width:number n:number"),
        ];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "number"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

pub fn align(_intp: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    let f_width = arguments.unwrap_number(0);
    let n = arguments.unwrap_number(1);

    if f_width.fract() != 0.0 {
        return Err(ErrorKind::InvalidInteger(f_width).into());
    }

    let width: usize = f_width
        .round()
        .approx()
        .map_err(|_| ErrorKind::InvalidInteger(f_width))?;

    Ok(format!("{n:0>width$}").into())
}

pub fn parse(_intp: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    let string = arguments.unwrap_string(0);

    let result: Result<f64> = string
        .trim()
        .parse()
        .map_err(|_| ErrorKind::ParseNumberError(string).into());

    Ok(Value::Number(result?))
}
