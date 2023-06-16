use crate::error::{Error, ErrorKind};
use crate::interpreter::Value;
use crate::Result;

#[derive(Debug, Clone)]
pub enum ParameterType {
    TypedList(Box<ParameterType>),
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
            ParameterType::Function => {
                matches!(value, Value::Function(_) | Value::Builtin(_))
            }
            ParameterType::List => matches!(value, Value::List(_)),
            ParameterType::Number => matches!(value, Value::Number(_)),
            ParameterType::String => matches!(value, Value::String(_)),
            ParameterType::Boolean => matches!(value, Value::Boolean(_)),
            ParameterType::Symbol => matches!(value, Value::Symbol(_)),
            ParameterType::Keyword => matches!(value, Value::Keyword(_)),
            ParameterType::Nil => matches!(value, Value::Nil),
            ParameterType::TypedList(ptype) => {
                if let Value::List(values) = value {
                    values.iter().all(|v| ptype.value_is_type(v))
                } else {
                    false
                }
            }
        }
    }
}

impl TryFrom<&str> for ParameterType {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        let ptype = match string {
            "string" => Self::String,
            "number" => Self::Number,
            "list" => Self::List,
            "bool" | "boolean" => Self::Boolean,
            other => {
                return Err(
                    ErrorKind::InvalidParameterType(other.to_owned()).into()
                )
            }
        };

        Ok(ptype)
    }
}
