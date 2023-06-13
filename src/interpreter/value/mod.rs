mod builtin;
mod function;

pub use builtin::Builtin;
pub use function::Function;

use super::Interpreter;
use crate::Result;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    Keyword(String),
    List(Vec<Self>),
    Builtin(Builtin),
    Function(Function),
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        #[allow(clippy::match_same_arms)]
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(left), Value::Boolean(right)) => left == right,
            (Value::Number(left), Value::Number(right)) => {
                (*left - *right).abs() < f64::EPSILON
            }
            (Value::String(left), Value::String(right)) => left == right,
            (Value::Symbol(left), Value::Symbol(right)) => left == right,
            (Value::List(left), Value::List(right)) => left == right,
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

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(boolean) => *boolean,
            _ => true,
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    pub fn as_callable(&self) -> Option<&dyn Callable> {
        match self {
            Value::Builtin(b) => Some(b),
            Value::Function(f) => Some(f),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(boolean) => write!(f, "{boolean}"),
            Value::Number(number) => write!(f, "{number}"),
            Value::String(string) => write!(f, "{string}"),
            Value::Symbol(symbol) => write!(f, "{symbol}"),
            Value::Keyword(keyword) => write!(f, ":{keyword}"),
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
            Value::Builtin(builtin) => builtin.fmt(f),
            Value::Function(function) => {
                write!(f, "<function ({} args)>", function.parameters.len())
            }
        }
    }
}

pub trait Callable: std::fmt::Debug + std::fmt::Display {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value>;
}
