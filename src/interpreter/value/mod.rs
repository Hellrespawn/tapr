mod function;

pub use function::Function;

use super::arguments::Arguments;
use super::environment::Environment;
use super::native::NativeFunction;
use super::Interpreter;
use crate::{Node, Parameters, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Value {
    Function(Arc<dyn Callable<Result<Value>>>),
    Macro(Arc<dyn Callable<Result<Node>>>),
    Module(Environment),
    Map {
        mutable: bool,
        map: HashMap<Value, Value>,
    },
    List {
        mutable: bool,
        list: Vec<Value>,
    },
    Number(f64),
    String {
        mutable: bool,
        string: String,
    },
    Symbol(String),
    Keyword(String),
    Boolean(bool),
    Nil,
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
            Value::String { mutable, string } => {
                format!("\"{}{string}\"", if *mutable { "@" } else { "" })
            }
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

impl From<NativeFunction> for Value {
    fn from(value: NativeFunction) -> Self {
        Value::Function(Arc::new(value))
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
            }
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Module(environment) => {
                write!(f, "<module ({})>", environment.len())
            }
            Self::Function(function) => function.fmt(f),
            Self::Macro(macro_) => macro_.fmt(f),
            Self::List { mutable, list } => {
                write!(
                    f,
                    "{}[{}]",
                    if *mutable { "@" } else { "" },
                    list.iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Self::Map { mutable, map } => {
                write!(
                    f,
                    "{}{{{}}}",
                    if *mutable { "@" } else { "" },
                    map.iter()
                        .map(|(key, value)| { format!("{key} => {value}") })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Number(number) => write!(f, "{number}"),
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Buffer(string) => write!(f, "@\"{string}\""),
            Self::Symbol(symbol) => write!(f, "{symbol}"),
            Self::Keyword(keyword) => write!(f, ":{keyword}"),
            Self::Boolean(bool) => write!(f, "{bool}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

pub enum CallableType {
    Native,
    Function,
    Macro,
    SpecialForm,
}

impl std::fmt::Display for CallableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CallableType::Native => "native fn",
                CallableType::Function => "fn",
                CallableType::Macro => "macro",
                CallableType::SpecialForm => "special form",
            }
        )
    }
}

pub trait Callable<T>: Send + Sync {
    fn call(&self, intp: &mut Interpreter, arguments: Arguments) -> T;

    fn arity(&self) -> usize;

    fn callable_type(&self) -> CallableType;

    fn parameters(&self) -> Parameters;
}

impl<T> std::fmt::Display for dyn Callable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}{}> ", self.callable_type(), self.parameters())
    }
}

impl<T> std::fmt::Debug for dyn Callable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
