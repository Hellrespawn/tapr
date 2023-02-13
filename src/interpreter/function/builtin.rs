use super::Function;
use crate::interpreter::Value;
use crate::{Error, Result};

#[derive(Debug, Copy, Clone)]
pub enum ArithmeticKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct ArithmeticFunction {
    kind: ArithmeticKind,
}

impl ArithmeticFunction {
    pub fn new(kind: ArithmeticKind) -> Self {
        Self { kind }
    }

    fn check_num_args(num_args: usize) -> Result<()> {
        if num_args >= 2 {
            Ok(())
        } else {
            Err(Error::WrongAmountOfArgs {
                expected: ">= 2",
                actual: num_args,
            })
        }
    }

    pub fn op(&self, left: &Value, right: &Value) -> Result<Value> {
        match (self.kind, left, right) {
            (
                ArithmeticKind::Add,
                Value::Number(left),
                Value::Number(right),
            ) => Ok(Value::Number(left + right)),
            (
                ArithmeticKind::Add,
                Value::String(left),
                Value::String(right),
            ) => Ok(Value::String(format!("{left}{right}"))),
            (
                ArithmeticKind::Subtract,
                Value::Number(left),
                Value::Number(right),
            ) => Ok(Value::Number(left - right)),
            (
                ArithmeticKind::Multiply,
                Value::Number(left),
                Value::Number(right),
            ) => Ok(Value::Number(left * right)),
            (
                ArithmeticKind::Divide,
                Value::Number(left),
                Value::Number(right),
            ) => Ok(Value::Number(left / right)),
            _ => Err(Error::InvalidArgument {
                expected: "Number or String",
                value: right.clone(),
            }),
        }
    }
}

impl Function for ArithmeticFunction {
    fn call(&self, args: &[Value]) -> Result<Value> {
        ArithmeticFunction::check_num_args(args.len())?;

        let mut iter = args.iter();

        let mut acc = iter.next().expect("More than two elements.").clone();

        for value in iter {
            acc = self.op(&acc, value)?;
        }

        Ok(acc)
    }
}
