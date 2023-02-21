use crate::error::{Error, ErrorKind};
use crate::interpreter::function::{Arguments, Function};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Node;
use crate::Result;

#[derive(Debug)]
pub struct TailFunction;

impl TailFunction {
    const ARGUMENTS: Arguments = Arguments::Fixed(1);
}

impl Function for TailFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_arg = TailFunction::ARGUMENTS
            .evaluate(intp, arguments_nodes)?
            .pop()
            .expect("one argument.");

        if let Value::List(list) = evaluated_arg {
            if list.is_empty() {
                Err(Error::without_location(ErrorKind::TailOnEmptyList))
            } else {
                Ok(list.last().cloned().expect("list not be empty"))
            }
        } else {
            Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "List",
                values: vec![evaluated_arg],
            }))
        }
    }

    fn name(&self) -> &str {
        "tail"
    }
}
