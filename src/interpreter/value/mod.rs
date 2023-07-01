mod callable;
mod function;

pub use callable::{Callable, CallableType};
pub use function::Function;

use super::environment::Environment;
use super::native::NativeFunction;
use crate::{Node, NodeData};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Value {
    Function(Arc<dyn Callable<Value>>),
    Macro(Arc<dyn Callable<Node>>),
    Module(Environment),
    Node(NodeData<Value>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        !self.is_falsy()
    }

    pub fn is_falsy(&self) -> bool {
        matches!(self, Value::Node(NodeData::False | NodeData::Nil))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Node(NodeData::Nil))
    }

    pub(crate) fn table(map: HashMap<Value, Value>) -> Value {
        Value::Node(NodeData::Table(map))
    }

    pub(crate) fn struct_(map: HashMap<Value, Value>) -> Value {
        Value::Node(NodeData::Struct(map))
    }

    pub(crate) fn p_array(values: Vec<Value>) -> Value {
        Value::Node(NodeData::PArray(values))
    }

    pub(crate) fn b_array(values: Vec<Value>) -> Value {
        Value::Node(NodeData::BArray(values))
    }

    pub(crate) fn p_tuple(values: Vec<Value>) -> Value {
        Value::Node(NodeData::PTuple(values))
    }

    pub(crate) fn b_tuple(values: Vec<Value>) -> Value {
        Value::Node(NodeData::BTuple(values))
    }

    pub(crate) fn number(number: f64) -> Value {
        Value::Node(NodeData::Number(number))
    }

    pub(crate) fn string(string: String) -> Value {
        Value::Node(NodeData::String(string))
    }

    pub(crate) fn buffer(string: String) -> Value {
        Value::Node(NodeData::Buffer(string))
    }

    pub(crate) fn keyword(keyword: String) -> Value {
        Value::Node(NodeData::Keyword(keyword))
    }

    pub(crate) fn symbol(symbol: String) -> Value {
        Value::Node(NodeData::Symbol(symbol))
    }

    pub fn bool(bool: bool) -> Self {
        if bool {
            Value::Node(NodeData::True)
        } else {
            Value::Node(NodeData::False)
        }
    }

    pub fn nil() -> Self {
        Value::Node(NodeData::Nil)
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::Node(
                NodeData::Buffer(string) | NodeData::String(string),
            ) => Some(string.as_str()),
            _ => None,
        }
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
            (Value::Node(lhs), Value::Node(rhs)) => lhs.eq(rhs),
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
            Value::Node(data) => data.hash(state),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        #[allow(clippy::match_same_arms)]
        match (self, other) {
            (Value::Node(lhs), Value::Node(rhs)) => lhs.partial_cmp(rhs),
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
            Self::Node(data) => data.fmt(f),
        }
    }
}
