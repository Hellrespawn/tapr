#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]
use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::location::Location;
use crate::Result;

pub struct Debug;

impl NativeModule for Debug {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> =
            vec![("env", env, ""), ("lsmod", lsmod, "m:module")];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "debug"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn env(
    _location: Location,
    intp: &mut Interpreter,
    _arguments: Arguments<Value>,
) -> Result<Value> {
    println!("{}", intp.environment);

    Ok(Value::nil())
}

fn lsmod(
    _location: Location,
    _intp: &mut Interpreter,
    arguments: Arguments<Value>,
) -> Result<Value> {
    let environment = arguments.unwrap_module(0);

    println!("{environment}");

    Ok(Value::nil())
}
