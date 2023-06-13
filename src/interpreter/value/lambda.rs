use super::Value;
use crate::interpreter::parameters::Parameters;
use crate::interpreter::{Arguments, Interpreter};
use crate::parser::ast::Node;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Lambda {
    pub parameters: Parameters,
    pub body: Vec<Node>,
}

impl Lambda {
    pub fn new(parameters: Parameters, body: Vec<Node>) -> Self {
        Self { parameters, body }
    }

    pub fn call(
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
