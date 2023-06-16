use super::environment::Environment;
use super::value::Callable;
use super::{Arguments, Interpreter, Parameters, Value};
use crate::Result;
use once_cell::sync::Lazy;

mod core;
mod debug;

pub static NATIVE_ENVIRONMENT: Lazy<Environment> = Lazy::new(|| {
    let mut environment = core::get_core_environment();

    environment
        .insert(
            "debug".to_owned(),
            Value::Module(debug::get_debug_environment()),
        )
        .expect("Unable to add debug module to core environment.");

    environment
});

pub type NativeFunctionTuple = (&'static str, NativeFunctionImpl, &'static str);

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

fn tuple_to_value(tuple: NativeFunctionTuple) -> Value {
    NativeFunction::new(
        tuple.0,
        tuple.1,
        tuple
            .2
            .try_into()
            .expect("Native function should have valid parameters-string."),
    )
    .into()
}
