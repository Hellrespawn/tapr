#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]

use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct Core;

impl NativeModule for Core {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("println", println, "& s"),
            ("print", print, "& s"),
            ("is-nil", is_nil, "v"),
        ];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "core"
    }

    fn is_core_module(&self) -> bool {
        true
    }
}

fn println(
    _intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    println!();

    Ok(Value::nil())
}

fn print(
    _intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    Ok(Value::nil())
}

fn is_nil(_: &mut Interpreter, arguments: Arguments<Value>) -> Result<Value> {
    let argument = arguments.unwrap(0);

    Ok(Value::bool(argument.is_nil()))
}
