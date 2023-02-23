use super::Value;
use crate::interpreter::callable::Callable;
use crate::interpreter::parameters::Parameters;
use crate::interpreter::Interpreter;
use crate::parser::ast::Node;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Parameters,
    pub node: Box<Node>,
}

impl Function {
    pub fn new(name: String, parameters: Parameters, node: Box<Node>) -> Self {
        Self {
            name,
            parameters,
            node,
        }
    }
}

impl Callable for Function {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let arguments =
            self.parameters.evaluate_arguments(intp, argument_nodes)?;

        intp.enter_scope();

        // Insert arguments into scope
        for (argument, parameter) in
            arguments.into_iter().zip(&self.parameters.parameters)
        {
            intp.environment.insert(parameter.name.clone(), argument);
        }

        let value = self.node.accept(intp)?;

        intp.exit_scope();

        Ok(value)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
