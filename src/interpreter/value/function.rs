use super::{Callable, CallableType, Value};
use crate::interpreter::environment::Environment;
use crate::interpreter::{Arguments, Interpreter};
use crate::parser::parameters::Parameters;
use crate::{Node, Result};

#[derive(Debug, Clone)]
pub struct Function {
    pub parameters: Parameters,
    pub body: Vec<Node>,
}

impl Function {
    pub fn new(parameters: Parameters, body: Vec<Node>) -> Self {
        Self { parameters, body }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function ({})>", self.parameters.len())
    }
}

impl Callable<Value> for Function {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Arguments<Value>,
    ) -> Result<Value> {
        let mut function_environment = Environment::new();
        arguments.add_to_env(&mut function_environment)?;

        intp.push_environment(function_environment);

        let mut values = self
            .body
            .iter()
            .map(|n| n.accept(intp))
            .collect::<Result<Vec<_>>>()?;

        intp.pop_environment();

        Ok(values.pop().unwrap_or(Value::Nil))
    }

    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn callable_type(&self) -> CallableType {
        CallableType::Function
    }

    fn parameters(&self) -> Parameters {
        self.parameters.clone()
    }
}
