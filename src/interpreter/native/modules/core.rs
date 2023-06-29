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

fn println(intp: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    println!();

    Ok(Value::Nil)
}

fn print(intp: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    Ok(Value::Nil)
}

fn is_nil(_: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    let argument = arguments.unwrap(0);

    Ok(matches!(argument, Value::Nil).into())
}
