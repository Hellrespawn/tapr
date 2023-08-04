use crate::callable::{NativeFunction, NativeMacro};
use crate::location::Location;
use crate::visitor::Visitor;
use crate::{Callable, Environment};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeSource {
    Location(Location),
    Node(Arc<Node>),
    Unknown,
}

impl NodeSource {
    pub fn location(&self) -> Option<Location> {
        match self {
            NodeSource::Location(location) => Some(*location),
            NodeSource::Node(node) => node.source().location(),
            NodeSource::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    source: NodeSource,
    data: NodeData,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data())
    }
}

impl From<NativeFunction> for Node {
    fn from(value: NativeFunction) -> Self {
        Self::unknown(NodeData::Function(Arc::new(value)))
    }
}

impl From<NativeMacro> for Node {
    fn from(value: NativeMacro) -> Self {
        Self::unknown(NodeData::Macro(Arc::new(value)))
    }
}

impl From<Environment> for Node {
    fn from(value: Environment) -> Self {
        Self::unknown(NodeData::Module(value))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl Node {
    pub fn new(source: NodeSource, data: NodeData) -> Self {
        Self { source, data }
    }

    pub fn with_location(location: Location, data: NodeData) -> Self {
        Self {
            source: NodeSource::Location(location),
            data,
        }
    }

    pub fn from_node(node: Node, data: NodeData) -> Self {
        Self {
            source: NodeSource::Node(Arc::new(node)),
            data,
        }
    }

    pub fn unknown(data: NodeData) -> Self {
        Self {
            source: NodeSource::Unknown,
            data,
        }
    }

    pub fn accept<T: std::fmt::Debug>(
        &self,
        visitor: &mut dyn Visitor<T>,
    ) -> T {
        visitor.visit_node(self)
    }

    pub fn data(&self) -> &NodeData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut NodeData {
        &mut self.data
    }

    pub fn source(&self) -> NodeSource {
        self.source.clone()
    }

    pub fn is_unquote(&self) -> bool {
        if let NodeData::Symbol(symbol) = self.data() {
            if symbol == "unquote" {
                return true;
            }
        }

        false
    }

    pub fn is_truthy(&self) -> bool {
        match self.data() {
            NodeData::Bool(bool) => *bool,
            _ => true,
        }
    }

    pub fn is_falsy(&self) -> bool {
        !self.is_truthy()
    }

    pub fn is_nil(&self) -> bool {
        matches!(self.data(), NodeData::Nil)
    }

    pub fn as_string(&self) -> Option<&str> {
        if let NodeData::String(string) | NodeData::Buffer(string) = self.data()
        {
            Some(string)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeData {
    // Interpreter only
    Function(Arc<dyn Callable>),
    Macro(Arc<dyn Callable>),
    Module(Environment),
    // Parser only
    Main(Vec<Node>),
    Table(HashMap<Node, Node>),
    PArray(Vec<Node>),
    BArray(Vec<Node>),
    Struct(HashMap<Node, Node>),
    PTuple(Vec<Node>),
    BTuple(Vec<Node>),
    Number(f64),
    String(String),
    Buffer(String),
    Symbol(String),
    Keyword(String),
    Bool(bool),
    Nil,
}

impl std::fmt::Display for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Module(environment) => {
                write!(f, "<module ({})>", environment.len())
            }
            Self::Function(function) => function.fmt(f),
            Self::Macro(macro_) => macro_.fmt(f),
            Self::Main(nodes) => write!(
                f,
                "{}",
                nodes
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Table(map) => write!(
                f,
                "@{{{}}}",
                map.iter()
                    .map(|(key, value)| { format!("{key} => {value}") })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::PArray(nodes) => write!(
                f,
                "@({})",
                nodes
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::BArray(nodes) => write!(
                f,
                "@[{}]",
                nodes
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Struct(map) => write!(
                f,
                "{{{}}}",
                map.iter()
                    .map(|(key, value)| { format!("{key} => {value}") })
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::PTuple(nodes) => write!(
                f,
                "({})",
                nodes
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::BTuple(nodes) => write!(
                f,
                "[{}]",
                nodes
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Number(number) => write!(f, "{number}"),
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Buffer(string) => write!(f, "@\"{string}\""),
            Self::Symbol(symbol) => write!(f, "{symbol}"),
            Self::Keyword(keyword) => write!(f, ":{keyword}"),
            Self::Bool(bool) => write!(f, "{bool}"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeData::Main(lhs), NodeData::Main(rhs))
            | (NodeData::PArray(lhs), NodeData::PArray(rhs))
            | (NodeData::BArray(lhs), NodeData::BArray(rhs))
            | (NodeData::PTuple(lhs), NodeData::PTuple(rhs))
            | (NodeData::BTuple(lhs), NodeData::BTuple(rhs)) => lhs == rhs,
            (NodeData::Table(lhs), NodeData::Table(rhs))
            | (NodeData::Struct(lhs), NodeData::Struct(rhs)) => lhs == rhs,

            (NodeData::String(lhs), NodeData::String(rhs))
            | (NodeData::Buffer(lhs), NodeData::Buffer(rhs))
            | (NodeData::Symbol(lhs), NodeData::Symbol(rhs))
            | (NodeData::Keyword(lhs), NodeData::Keyword(rhs)) => lhs == rhs,

            (NodeData::Bool(lhs), NodeData::Bool(rhs)) => lhs == rhs,

            (NodeData::Nil, NodeData::Nil) => true,
            (NodeData::Number(lhs), NodeData::Number(rhs)) => {
                lhs.to_bits() == rhs.to_bits()
            }
            _ => false,
        }
    }
}

impl Eq for NodeData {}

impl PartialOrd for NodeData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (NodeData::Nil, NodeData::Nil) => Some(Ordering::Equal),
            (NodeData::Number(lhs), NodeData::Number(rhs)) => {
                lhs.partial_cmp(rhs)
            }
            (NodeData::String(lhs), NodeData::String(rhs)) => {
                lhs.partial_cmp(rhs)
            }
            (NodeData::Symbol(left), NodeData::Symbol(right)) => {
                left.partial_cmp(right)
            }
            _ => None,
        }
    }
}

impl std::hash::Hash for NodeData {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Function(_) => unreachable!("Unable to hash callable"),
            Self::Macro(_) => unreachable!("Unable to hash macro"),
            Self::Module(env) => env.hash(state),
            Self::Main(nodes)
            | Self::PArray(nodes)
            | Self::PTuple(nodes)
            | Self::BArray(nodes)
            | Self::BTuple(nodes) => nodes.hash(state),
            Self::Table(map) | Self::Struct(map) => {
                map.iter().collect::<Vec<_>>().hash(state);
            }
            Self::Number(number) => number.to_bits().hash(state),
            Self::String(string)
            | Self::Buffer(string)
            | Self::Symbol(string)
            | Self::Keyword(string) => string.hash(state),
            _ => (),
        }
    }
}
