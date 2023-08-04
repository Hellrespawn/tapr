use crate::error::{Error, ErrorKind};
use crate::{Node, NodeData, Result};

#[derive(Debug, Clone, Copy)]
pub enum ParameterType {
    Map,
    List,
    Module,
    Function,
    Macro,
    Number,
    String,
    Bool,
    Symbol,
    Keyword,
    Nil,
}

impl ParameterType {
    pub fn node_is_type(&self, node: &Node) -> bool {
        matches!(
            (self, node.data()),
            (
                ParameterType::Map,
                NodeData::Struct(..) | NodeData::Table(..),
            ) | (
                ParameterType::List,
                NodeData::BArray(..)
                    | NodeData::BTuple(..)
                    | NodeData::PArray(..)
                    | NodeData::PTuple(..)
            ) | (ParameterType::Module, NodeData::Module(..))
                | (ParameterType::Function, NodeData::Function(..))
                | (ParameterType::Macro, NodeData::Macro(..))
                | (ParameterType::Number, NodeData::Number(..))
                | (
                    ParameterType::String,
                    NodeData::String(..) | NodeData::Buffer(..)
                )
                | (ParameterType::Bool, NodeData::Bool(..))
                | (ParameterType::Symbol, NodeData::Symbol(..))
                | (ParameterType::Keyword, NodeData::Keyword(..))
                | (ParameterType::Nil, NodeData::Nil)
        )
    }
}

impl TryFrom<&str> for ParameterType {
    type Error = Error;

    fn try_from(string: &str) -> Result<Self> {
        let ptype = match string {
            "bool" => ParameterType::Bool,
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
            ParameterType::Macro => write!(f, "macro"),
            ParameterType::Number => write!(f, "number"),
            ParameterType::String => write!(f, "string"),
            ParameterType::Bool => write!(f, "bool"),
            ParameterType::Symbol => write!(f, "symbol"),
            ParameterType::Keyword => write!(f, "keyword"),
            ParameterType::Nil => write!(f, "nil"),
            ParameterType::Map => write!(f, "map"),
        }
    }
}
