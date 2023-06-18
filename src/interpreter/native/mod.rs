use super::environment::Environment;
use super::value::Callable;
use super::{Arguments, Interpreter, Parameters, Value};
use crate::Result;
use once_cell::sync::Lazy;

mod modules;

pub static NATIVE_ENVIRONMENT: Lazy<Environment> = Lazy::new(|| {
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
                .insert(module.name().to_owned(), module.environment().into())
                .unwrap_or_else(|_| {
                    panic!("Unable to insert '{}' module.", module.name())
                });
        }
    }

    environment
});

pub type NativeFunctionImpl =
    fn(intp: &mut Interpreter, arguments: &Arguments) -> Result<Value>;

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
            arguments: &Arguments,
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

impl Callable for NativeFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value> {
        let arguments = Arguments::new(&self.parameters, arguments)?;

        (self.function)(intp, &arguments)
    }

    fn arity(&self) -> usize {
        self.parameters.len()
    }
}
