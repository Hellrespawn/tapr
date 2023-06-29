use super::environment::Environment;
use super::value::{Callable, CallableType};
use super::{Arguments, Interpreter, Value};
use crate::{Parameters, Result};

mod modules;

pub fn get_native_environment() -> Environment {
    let mut environment = Environment::new();

    for module in modules::get_modules() {
        if module.is_core_module() {
            environment
                .merge_values(module.environment())
                .unwrap_or_else(|_| {
                    panic!("Unable to merge core '{}' module.", module.name())
                });
        } else {
            environment
                .def(module.name().to_owned(), module.environment().into())
                .unwrap_or_else(|_| {
                    panic!("Unable to insert '{}' module.", module.name())
                });
        }
    }

    environment
}

pub type NativeFunctionImpl =
    fn(intp: &mut Interpreter, arguments: Arguments) -> Result<Value>;

#[derive(Debug, Clone)]
pub struct NativeFunction {
    name: &'static str,
    function: NativeFunctionImpl,
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

impl std::fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function {} {}>", self.name, self.parameters)
    }
}

impl Callable<Result<Value>> for NativeFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Arguments,
    ) -> Result<Value> {
        (self.function)(intp, arguments)
    }

    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn callable_type(&self) -> CallableType {
        CallableType::Native
    }

    fn parameters(&self) -> Parameters {
        self.parameters.clone()
    }
}
