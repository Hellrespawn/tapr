use super::Value;
use crate::interpreter::parameters::Parameters;
use crate::interpreter::{Arguments, Interpreter};
use crate::parser::ast::Expression;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Lambda {
    pub parameters: Parameters,
    pub expression: Box<Expression>,
}

impl Lambda {
    pub fn new(parameters: Parameters, expression: Box<Expression>) -> Self {
        Self {
            parameters,
            expression,
        }
    }

    pub fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value> {
        let arguments = Arguments::new(&self.parameters, arguments)?;

        arguments.add_to_env(&mut intp.environment)?;

        let value = self.expression.accept(intp)?;

        Ok(value)
    }
}
