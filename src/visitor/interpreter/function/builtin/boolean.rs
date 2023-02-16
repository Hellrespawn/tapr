use crate::parser::ast::Node;
use crate::visitor::interpreter::function::{Arguments, Function};
use crate::visitor::interpreter::{Interpreter, Value};
use crate::Result;

#[derive(Debug, Copy, Clone)]
pub enum BooleanOp {
    Greater,
    GreaterOrEqual,
    Equal,
    LessOrEqual,
    Less,
}

pub struct BooleanFunction {
    operator: BooleanOp,
    arguments: Arguments,
}

impl BooleanFunction {
    pub fn new(operator: BooleanOp, arguments: usize) -> Self {
        Self {
            operator,
            arguments: Arguments::Minimum(arguments),
        }
    }

    pub fn op(&self, left: &Value, right: &Value) -> bool {
        match self.operator {
            BooleanOp::Greater => left > right,
            BooleanOp::GreaterOrEqual => left >= right,
            BooleanOp::Equal => left == right,
            BooleanOp::LessOrEqual => left <= right,
            BooleanOp::Less => left < right,
        }
    }
}

impl Function for BooleanFunction {
    fn call(
        &self,
        intp: &mut Interpreter,
        argument_nodes: &[Node],
    ) -> Result<Value> {
        let evaluated_args = self.arguments.evaluate(intp, argument_nodes)?;

        let result = evaluated_args.windows(2).all(|window| {
            let left = &window[0];
            let right = &window[1];

            self.op(left, right)
        });

        Ok(Value::Boolean(result))
    }
}
