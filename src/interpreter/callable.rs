use super::{Interpreter, Value};
use crate::error::{Error, ErrorKind};
use crate::parser::ast::Node;
use crate::Result;

pub trait Callable: std::fmt::Debug {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value>;

    fn name(&self) -> &str;
}

#[derive(Debug, Copy, Clone)]
pub enum Arguments {
    Fixed(usize),
    Minimum(usize),
    // Optional(usize),
    // Composite(Vec<Arguments>)
    // TODO handle optional arguments for functions.
}

impl Arguments {
    pub fn check_amount(&self, number_of_arguments: usize) -> Result<()> {
        let condition = match self {
            Arguments::Fixed(arguments) => number_of_arguments == *arguments,
            Arguments::Minimum(arguments) => number_of_arguments >= *arguments,
        };

        if condition {
            Ok(())
        } else {
            Err(Error::without_location(ErrorKind::WrongAmountOfArgs {
                expected: match self {
                    Arguments::Fixed(args) => format!(">= {args}"),
                    Arguments::Minimum(args) => format!("{args}"),
                },
                actual: number_of_arguments,
            }))
        }
    }

    pub fn evaluate(
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
