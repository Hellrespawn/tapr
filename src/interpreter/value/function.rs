use super::Value;
use crate::error::{Error, ErrorKind};
use crate::interpreter::function::Function;
use crate::interpreter::Interpreter;
use crate::parser::ast::Node;
use crate::Result;

#[derive(Debug, Clone)]
pub struct FunctionValue {
    pub name: String,
    pub parameters: Vec<String>,
    pub node: Box<Node>,
}

impl FunctionValue {
    pub fn new(name: String, parameters: Vec<String>, node: Box<Node>) -> Self {
        Self {
            name,
            parameters,
            node,
        }
    }

    fn check_arguments(&self, argument_nodes: &[Node]) -> Result<()> {
        if argument_nodes.len() == self.parameters.len() {
            Ok(())
        } else {
            Err(Error::without_location(ErrorKind::WrongAmountOfArgs {
                expected: self.parameters.len().to_string(),
                actual: argument_nodes.len(),
            }))
        }
    }
}

impl Function for FunctionValue {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        self.check_arguments(argument_nodes)?;

        // Evaluate arguments
        let argument_values = argument_nodes
            .iter()
            .map(|node| node.accept(intp))
            .collect::<Result<Vec<_>>>()?;

        intp.enter_scope();

        // Insert arguments into scope
        for (value, argument) in
            argument_values.into_iter().zip(&self.parameters)
        {
            intp.environment.insert(argument.clone(), value);
        }

        let value = self.node.accept(intp)?;

        intp.exit_scope();

        Ok(value)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
