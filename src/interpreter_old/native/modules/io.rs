#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct Io;

impl NativeModule for Io {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![("read", read, "")];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "io"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

pub fn read(_: &mut Interpreter, _: Arguments) -> Result<Value> {
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.into())
}
