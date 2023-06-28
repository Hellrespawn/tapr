use crate::error::{Error, ErrorKind};
use crate::{Node, Result};

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
    pub fn node_is_type(&self, node: &Node) -> bool {
        todo!()
        // match self {
        //     ParameterType::Module => {
        //         matches!(node, Value::Module { .. })
        //     }
        //     ParameterType::Function => matches!(node, Value::Callable(_)),
        //     ParameterType::List => matches!(node, Value::List(_)),
        //     ParameterType::Number => matches!(node, Value::Number(_)),
        //     ParameterType::String => matches!(node, Value::String(_)),
        //     ParameterType::Boolean => matches!(node, Value::Boolean(_)),
        //     ParameterType::Symbol => matches!(node, Value::Symbol(_)),
        //     ParameterType::Keyword => matches!(node, Value::Keyword(_)),
        //     ParameterType::Nil => matches!(node, Value::Nil),
        //     ParameterType::TypedList(ptype) => {
        //         if let Value::List(values) = node {
        //             values.iter().all(|v| ptype.value_is_type(v))
        //         } else {
        //             false
        //         }
        //     }
        // }
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
            ParameterType::TypedList(_) | ParameterType::List => {
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
        }
    }
}
