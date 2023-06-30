mod callable;
mod function;

pub use callable::{Callable, CallableType};
pub use function::Function;

use super::environment::Environment;
use super::native::NativeFunction;
use crate::Node;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Value {
    Function(Arc<dyn Callable<Value>>),
    Macro(Arc<dyn Callable<Node>>),
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
    pub fn string(string: String) -> Self {
        Self::String {
            mutable: false,
            string,
        }
    }

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

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::string(value)
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
            (
                Value::String { string: lhs, .. },
                Value::String { string: rhs, .. },
            ) => lhs == rhs,
            (Value::Symbol(left), Value::Symbol(right)) => left == right,
            _ => false,
        }
    }
}

impl Eq for Value {}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Value::Function(_) => unreachable!("Unable to hash callable"),
            Value::Macro(_) => unreachable!("Unable to hash macro"),
            Value::Module(env) => env.hash(state),
            Value::Map { mutable, map } => {
                mutable.hash(state);
                map.iter().collect::<Vec<_>>().hash(state);
            }
            Value::List { mutable, list } => {
                mutable.hash(state);
                list.hash(state);
            }
            Value::Number(number) => number.to_bits().hash(state),
            Value::String { mutable, string } => {
                mutable.hash(state);
                string.hash(state);
            }
            Value::Symbol(string) | Value::Keyword(string) => {
                string.hash(state);
            }
            Value::Boolean(bool) => bool.hash(state),
            Value::Nil => (),
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
            (
                Value::String { string: lhs, .. },
                Value::String { string: rhs, .. },
            ) => lhs.partial_cmp(rhs),
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
            Self::String { mutable, string } => {
                write!(f, "{}\"{string}\"", if *mutable { "@" } else { "" })
            }
            Self::Symbol(symbol) => write!(f, "{symbol}"),
            Self::Keyword(keyword) => write!(f, ":{keyword}"),
            Self::Boolean(bool) => write!(f, "{bool}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
