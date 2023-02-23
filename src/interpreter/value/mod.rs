mod function;

pub use function::Function;

use super::callable::Callable;
use crate::error::{Error, ErrorKind};
use crate::Result;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Self>),
    Function(Rc<dyn Callable>),
}

impl From<Function> for Value {
    fn from(value: Function) -> Self {
        Value::Function(Rc::new(value))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        #[allow(clippy::match_same_arms)]
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(left), Value::Boolean(right)) => left == right,
            (Value::Number(left), Value::Number(right)) => left == right,
            (Value::String(left), Value::String(right)) => left == right,
            (Value::Symbol(left), Value::Symbol(right)) => left == right,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        #[allow(clippy::match_same_arms)]
        match (self, other) {
            (Value::Nil, Value::Nil) => Some(Ordering::Equal),
            (Value::Boolean(left), Value::Boolean(right)) => {
                left.partial_cmp(right)
            }
            (Value::Number(left), Value::Number(right)) => {
                left.partial_cmp(right)
            }
            (Value::String(left), Value::String(right)) => {
                left.partial_cmp(right)
            }
            (Value::Symbol(left), Value::Symbol(right)) => {
                left.partial_cmp(right)
            }
            _ => None,
        }
    }
}

impl std::ops::Add for Value {
    type Output = Result<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(Value::Number(left + right))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(Value::String(format!("{left}{right}")))
            }
            _ => Err(Error::without_location(ErrorKind::InvalidBinOp {
                op: "add",
                lhs: self,
                rhs,
            })),
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Result<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        if let (Value::Number(left), Value::Number(right)) = (&self, &rhs) {
            Ok(Value::Number(left - right))
        } else {
            Err(Error::without_location(ErrorKind::InvalidBinOp {
                op: "subtract",
                lhs: self,
                rhs,
            }))
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Result<Value>;

    fn mul(self, rhs: Self) -> Self::Output {
        if let (Value::Number(left), Value::Number(right)) = (&self, &rhs) {
            Ok(Value::Number(left * right))
        } else {
            Err(Error::without_location(ErrorKind::InvalidBinOp {
                op: "multiply",
                lhs: self,
                rhs,
            }))
        }
    }
}

impl std::ops::Div for Value {
    type Output = Result<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        if let (Value::Number(left), Value::Number(right)) = (&self, &rhs) {
            Ok(Value::Number(left / right))
        } else {
            Err(Error::without_location(ErrorKind::InvalidBinOp {
                op: "divide",
                lhs: self,
                rhs,
            }))
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(boolean) => *boolean,
            Value::Number(number) => *number != 0.,
            Value::String(string) => !string.is_empty(),
            _ => true,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(boolean) => write!(f, "{boolean}"),
            Value::Number(number) => write!(f, "{number}"),
            Value::String(string) => write!(f, "\"{string}\""),
            Value::Symbol(symbol) => write!(f, "{symbol}"),
            Value::List(items) => {
                write!(
                    f,
                    "({})",
                    items
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Value::Function(function_value) => {
                write!(f, "<fn {}>", function_value.name())
            }
        }
    }
}
