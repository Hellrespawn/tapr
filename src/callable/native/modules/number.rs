#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::error::ErrorKind;
use crate::interpreter::{Arguments, Interpreter};
use crate::location::Location;
use crate::{Result, Environment, Node};
use conv::prelude::*;

pub struct Number;

impl NativeModule for Number {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("parse", parse, "s:string"),
            ("align", align, "width:number n:number"),
        ];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "number"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

pub fn align(
    _location: Location,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let f_width = arguments.unwrap_number(0);
    let n = arguments.unwrap_number(1);

    if f_width.fract() != 0.0 {
        return Err(ErrorKind::InvalidInteger(f_width).into());
    }

    let width: usize = f_width
        .round()
        .approx()
        .map_err(|_| ErrorKind::InvalidInteger(f_width))?;

    Ok(Node::string(format!("{n:0>width$}")))
}

pub fn parse(
    _location: Location,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let string = arguments.unwrap_string(0);

    let result: Result<f64> = string
        .trim()
        .parse()
        .map_err(|_| ErrorKind::ParseNumberError(string).into());

    Ok(Node::number(result?))
}
