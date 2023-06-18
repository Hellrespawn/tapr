use super::{tuples_to_environment, NativeFunctionTuple, NativeModule};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

pub struct Core;

impl NativeModule for Core {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> =
            vec![("println", println, "& s"), ("print", print, "& s")];

        tuples_to_environment(tuples, self.name())
    }

    fn name(&self) -> &'static str {
        "core"
    }

    fn is_core_module(&self) -> bool {
        true
    }
}

pub fn println(intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    for argument in arguments.arguments() {
        write!(intp.output, "{argument}")?;
    }

    writeln!(intp.output)?;

    Ok(Value::Nil)
}

pub fn print(intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    for argument in arguments.arguments() {
        write!(intp.output, "{argument}")?;
    }

    Ok(Value::Nil)
}
