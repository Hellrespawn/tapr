#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]
use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct Debug;

impl NativeModule for Debug {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> =
            vec![("env", env, ""), ("lsmod", lsmod, "m:module")];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "debug"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn env(intp: &mut Interpreter, _arguments: Arguments<Value>) -> Result<Value> {
    println!("{}", intp.environment);

    Ok(Value::nil())
}

fn lsmod(
    _intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let environment = arguments.unwrap_module(0);

    println!("{environment}");

    Ok(Value::nil())
}
