use super::{Callable, Value};
use crate::interpreter::parameters::Parameters;
use crate::interpreter::{Arguments, Interpreter};
use crate::parser::ast::Node;
use crate::Result;

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

impl Callable for Function {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value> {
        let arguments = Arguments::new(&self.parameters, arguments)?;

        intp.enter_scope();

        arguments.add_to_env(&mut intp.environment)?;

        let mut values = self
            .body
            .iter()
            .map(|n| n.accept(intp))
            .collect::<Result<Vec<_>>>()?;

        intp.exit_scope();

        Ok(values.pop().unwrap_or(Value::Nil))
    }
}
