mod function;

use std::cmp::Ordering;
use std::sync::Arc;

pub use function::Function;

use super::environment::Environment;
use super::native::NativeFunction;
use super::Interpreter;
use crate::{Arguments, Parameters, Result};

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Symbol(String),
    Keyword(String),
    List(Vec<Self>),
    Callable(Arc<dyn Callable>),
    Module(Environment),
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

    pub fn repl_repr(&self) -> String {
        match self {
            Value::String(string) => format!("\"{string}\""),
            other => other.to_string(),
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_owned())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<NativeFunction> for Value {
    fn from(value: NativeFunction) -> Self {
        Value::Callable(Arc::new(value))
    }
}

impl From<Environment> for Value {
    fn from(value: Environment) -> Self {
        Value::Module(value)
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
            },
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
            },
            (Value::Number(left), Value::Number(right)) => {
                left.partial_cmp(right)
            },
            (Value::String(left), Value::String(right)) => {
                left.partial_cmp(right)
            },
            (Value::Symbol(left), Value::Symbol(right)) => {
                left.partial_cmp(right)
            },
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
            },
            Value::Callable(callable) => callable.fmt(f),
            Value::Module(environment) => {
                write!(f, "<module ({})>", environment.len())
            },
        }
    }
}

pub enum CallableType {
    Native,
    Function,
}

impl std::fmt::Display for CallableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CallableType::Native => "native fn",
            CallableType::Function => "fn",
        })
    }
}

pub trait Callable: Send + Sync {
    fn call(
        &self,
        intp: &mut Interpreter,
        arguments: Arguments,
    ) -> Result<Value>;

    fn arity(&self) -> usize;

    fn callable_type(&self) -> CallableType;

    fn parameters(&self) -> Parameters;
}

impl std::fmt::Display for dyn Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}{}> ", self.callable_type(), self.parameters())
    }
}

impl std::fmt::Debug for dyn Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
