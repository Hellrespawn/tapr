use crate::interpreter::function::{Arguments, Function};
use crate::interpreter::Value;
use crate::Result;

#[derive(Debug, Copy, Clone)]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct ArithmeticFunction {
    operator: ArithmeticOp,
    arguments: Arguments,
}

impl ArithmeticFunction {
    pub fn new(operator: ArithmeticOp, arguments: usize) -> Self {
        Self {
            operator,
            arguments: Arguments::Minimum(arguments),
        }
    }

    pub fn op(&self, left: &Value, right: &Value) -> Result<Value> {
        match self.operator {
            ArithmeticOp::Add => left.clone() + right.clone(),
            ArithmeticOp::Subtract => left.clone() - right.clone(),
            ArithmeticOp::Multiply => left.clone() * right.clone(),
            ArithmeticOp::Divide => left.clone() / right.clone(),
        }
    }
}

impl Function for ArithmeticFunction {
    fn call(&self, arguments: &[Value]) -> Result<Value> {
        self.arguments.check_amount(arguments.len())?;

        let mut iter = arguments.iter();

        let mut acc = iter.next().expect("More than two elements.").clone();

        for value in iter {
            acc = self.op(&acc, value)?;
        }

        Ok(acc)
    }
}
