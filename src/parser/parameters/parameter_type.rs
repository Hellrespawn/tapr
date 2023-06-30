use crate::error::{Error, ErrorKind};
use crate::interpreter::Value;
use crate::Result;

#[derive(Debug, Clone, Copy)]
pub enum ParameterType {
    Map,
    List,
    Module,
    Function,
    Number,
    String,
    Boolean,
    Symbol,
    Keyword,
    Nil,
}

impl ParameterType {
    pub fn value_is_type(&self, value: &Value) -> bool {
        match self {
            ParameterType::Module => {
                matches!(value, Value::Module { .. })
            }
            ParameterType::Function => matches!(value, Value::Function(_)),
            ParameterType::List => matches!(value, Value::List { .. }),
            ParameterType::Number => matches!(value, Value::Number(_)),
            ParameterType::String => matches!(value, Value::String { .. }),
            ParameterType::Boolean => matches!(value, Value::Boolean(_)),
            ParameterType::Symbol => matches!(value, Value::Symbol(_)),
            ParameterType::Keyword => matches!(value, Value::Keyword(_)),
            ParameterType::Nil => matches!(value, Value::Nil),
            ParameterType::Map => {
                matches!(value, Value::Map { .. })
            }
        }
    }
}

impl TryFrom<&str> for ParameterType {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        let ptype = match string {
            "bool" => ParameterType::Boolean,
            "number" => ParameterType::Number,
            "symbol" => ParameterType::Symbol,
            "keyword" => ParameterType::Keyword,
            "string" => ParameterType::String,
            "list" => ParameterType::List,
            "module" => ParameterType::Module,
            "function" => ParameterType::Function,
            "nil" => ParameterType::Nil,
            other => {
                return Err(
                    ErrorKind::InvalidParameterType(other.to_owned()).into()
                )
            }
        };

        Ok(ptype)
    }
}

impl std::fmt::Display for ParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParameterType::List => {
                write!(f, "list")
            }
            ParameterType::Module => write!(f, "module"),
            ParameterType::Function => write!(f, "function"),
            ParameterType::Number => write!(f, "number"),
            ParameterType::String => write!(f, "string"),
            ParameterType::Boolean => write!(f, "bool"),
            ParameterType::Symbol => write!(f, "symbol"),
            ParameterType::Keyword => write!(f, "keyword"),
            ParameterType::Nil => write!(f, "nil"),
            ParameterType::Map => write!(f, "map"),
        }
    }
}
