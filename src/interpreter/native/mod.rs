use super::value::Callable;
use super::{Arguments, Interpreter, Parameters, Value};
use crate::Result;

mod arithmetic;
mod core;

pub struct NativeFunction {
    name: &'static str,
    function: fn(intp: &mut Interpreter, arguments: Arguments) -> Result<Value>,
    parameters: Parameters,
}

impl NativeFunction {
    pub fn new(
        name: &'static str,
        function: fn(
            intp: &mut Interpreter,
            arguments: Arguments,
        ) -> Result<Value>,
        parameters: Parameters,
    ) -> Self {
        Self {
            name,
            function,
            parameters,
        }
    }
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function {} ({})>", self.name, self.arity())
    }
}

impl Callable for NativeFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value> {
        let arguments = Arguments::new(&self.parameters, arguments)?;

        (self.function)(intp, arguments)
    }

    fn arity(&self) -> usize {
        self.parameters.len()
    }
}
