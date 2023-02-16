mod builtin;

pub use builtin::BUILTIN_FUNCTIONS;

use super::{Interpreter, Value};
use crate::parser::ast::Node;
use crate::{Error, Result};
pub trait Function: Sync + Send {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value>;
}

#[derive(Debug, Copy, Clone)]
pub enum Arguments {
    Fixed(usize),
    Minimum(usize),
}

impl Arguments {
    fn check_amount(&self, number_of_arguments: usize) -> Result<()> {
        let condition = match self {
            Arguments::Fixed(arguments) => number_of_arguments == *arguments,
            Arguments::Minimum(arguments) => number_of_arguments >= *arguments,
        };

        if condition {
            Ok(())
        } else {
            Err(Error::WrongAmountOfArgs {
                expected: match self {
                    Arguments::Fixed(args) => format!(">= {args}"),
                    Arguments::Minimum(args) => format!("{args}"),
                },
                actual: number_of_arguments,
            })
        }
    }

    fn evaluate(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Vec<Value>> {
        self.check_amount(argument_nodes.len())?;

        argument_nodes
            .iter()
            .map(|node| node.accept(intp))
            .collect::<Result<Vec<_>>>()
    }
}
