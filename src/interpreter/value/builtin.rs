use super::{Callable, Value};
use crate::interpreter::builtins::BuiltinFunction;
use crate::interpreter::{Interpreter, Parameters};
use crate::Result;

#[derive(Clone)]
pub struct Builtin {
    name: &'static str,
    function: BuiltinFunction,
    parameters: Parameters,
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::fmt::Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<builtin function {}>", self.name)
    }
}

impl Builtin {
    pub fn new(
        name: &'static str,
        function: BuiltinFunction,
        parameters: Parameters,
    ) -> Self {
        Self {
            name,
            function,
            parameters,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }
}

impl Callable for Builtin {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value> {
        (self.function)(intp, arguments)
    }

    fn arity(&self) -> usize {
        self.parameters.len()
    }
}
