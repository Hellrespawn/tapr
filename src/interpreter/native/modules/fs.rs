#![allow(clippy::unnecessary_wraps)]
use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct Fs;

impl NativeModule for Fs {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("read_to_string", read_to_string, "path:string"),
            ("write", write, "path:string body:string"),
        ];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "fs"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn read_to_string(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let path = arguments.unwrap_string(0);

    Ok(std::fs::read_to_string(path)?.into())
}

fn write(_: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    let path = arguments.unwrap_string(0);
    let body = arguments.unwrap_string(1);

    std::fs::write(path, body)?;

    Ok(Value::Nil)
}
