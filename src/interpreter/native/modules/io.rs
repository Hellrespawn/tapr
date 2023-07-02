#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::location::Location;
use crate::Result;

pub struct Io;

impl NativeModule for Io {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![("read", read, "")];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "io"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

pub fn read(
    _location: Location,
    _: &mut Interpreter,
    _: Arguments<Value>,
) -> Result<Value> {
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;

    Ok(Value::string(buffer))
}
