use super::Value;
use crate::interpreter::parameters::Parameters;
use crate::interpreter::Interpreter;
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
        argument_nodes: &[Expression],
    ) -> Result<Value> {
        let arguments =
            self.parameters.evaluate_arguments(intp, argument_nodes)?;

        // Insert arguments into scope
        for (argument, parameter) in
            arguments.into_iter().zip(&self.parameters.parameters)
        {
            intp.environment.insert(parameter.name.clone(), argument);
        }

        let value = self.expression.accept(intp)?;

        Ok(value)
    }
}
