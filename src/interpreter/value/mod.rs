mod function;

pub use function::Function;

use crate::error::{Error, ErrorKind};
use crate::Result;
use std::cmp::Ordering;
use std::rc::Rc;

use super::callable::Callable;

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
        #[allow(clippy::match_same_arms)]
        match (&self, &rhs) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(Value::Number(left + right))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(Value::String(format!("{left}{right}")))
            }
            _ => Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "Numbers or Strings",
                values: vec![self, rhs],
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
            Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "Numbers",
                values: vec![self, rhs],
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
            Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "Numbers",
                values: vec![self, rhs],
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
            Err(Error::without_location(ErrorKind::InvalidArguments {
                expected: "Numbers",
                values: vec![self, rhs],
            }))
        }
    }
}

impl Value {
    pub fn get_symbol(&self) -> Option<String> {
        if let Self::Symbol(string) = self {
            Some(string.clone())
        } else {
            None
        }
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if let Self::Boolean(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        if let Self::Number(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        if let Self::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_symbol(&self) -> Option<&str> {
        if let Self::Symbol(v) = self {
            Some(v.as_str())
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&[Self]> {
        if let Self::List(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_function(&self) -> Option<Rc<dyn Callable>> {
        if let Self::Function(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }

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
                write!(f, "(")?;

                for element in items {
                    write!(f, "{element} ")?;
                }

                write!(f, "\x08)")
            }
            Value::Function(function_value) => {
                write!(f, "<fn {}>", function_value.name())
            }
        }
    }
}
