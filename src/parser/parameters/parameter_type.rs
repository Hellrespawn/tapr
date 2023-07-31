use crate::error::{Error, ErrorKind};
use crate::{Node, NodeData, Result};

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
    pub fn node_is_type(&self, value: &Node) -> bool {
        todo!("Implement `node_is_type`")
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
            _ => {
                return Err(ErrorKind::Message(format!(
                    "Invalid parameter type: '{string}'"
                ))
                .into())
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
