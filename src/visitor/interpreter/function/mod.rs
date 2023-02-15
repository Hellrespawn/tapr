mod builtin;

pub use builtin::BUILTIN_FUNCTIONS;

use super::Value;
use crate::{Error, Result};
pub trait Function: Sync + Send {
    fn call(&self, args: &[Value]) -> Result<Value>;
}

#[derive(Debug, Copy, Clone)]
pub enum Arguments {
    Fixed(usize),
    Minimum(usize),
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
            Err(Error::WrongAmountOfArgs {
                expected: match self {
                    Arguments::Fixed(args) => format!(">= {args}"),
                    Arguments::Minimum(args) => format!("{args}"),
                },
                actual: number_of_arguments,
            })
        }
    }
}
