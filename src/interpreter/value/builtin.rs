use super::Value;
use crate::interpreter::builtins::BuiltinFunction;
use crate::interpreter::Interpreter;
use crate::Result;

#[derive(Clone)]
pub struct Builtin {
    name: &'static str,
    function: BuiltinFunction,
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
    pub fn new(name: &'static str, function: BuiltinFunction) -> Self {
        Self { name, function }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value> {
        (self.function)(intp, arguments)
    }
}
