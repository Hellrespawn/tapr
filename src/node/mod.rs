use self::callable::Callable;
use crate::location::Location;
use crate::visitor::Visitor;
use std::collections::HashMap;
use std::sync::Arc;

pub mod callable;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    location: Location,
    data: NodeData,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data())
    }
}

impl Node {
    pub fn new(location: Location, data: NodeData) -> Self {
        Self { location, data }
    }

    pub fn mock(data: NodeData) -> Self {
        Self {
            location: Location::new(0, 0),
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

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn as_callable(&self) -> Option<&dyn Callable> {
        if let NodeData::Callable(callable) = self.data() {
            Some(&**callable)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeData {
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
    True,
    False,
    Nil,
    // Internal
    Callable(Arc<dyn Callable>),
}

impl std::fmt::Display for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Main(nodes) => write!(f, "<main {}>", nodes.len()),
            Self::Table(map) => write!(f, "@{{{}}}", map.len()),
            Self::PArray(nodes) => write!(f, "<@({})>", nodes.len()),
            Self::BArray(nodes) => write!(f, "<@[{}]>", nodes.len()),
            Self::Struct(map) => write!(f, "{{{}}}", map.len()),
            Self::PTuple(nodes) => write!(f, "<({})>", nodes.len()),
            Self::BTuple(nodes) => write!(f, "<[{}]>", nodes.len()),
            Self::Number(number) => write!(f, "{number}"),
            Self::String(string) => write!(f, "\"{string}\""),
            Self::Buffer(string) => write!(f, "@\"{string}\""),
            Self::Symbol(symbol) => write!(f, "{symbol}"),
            Self::Keyword(keyword) => write!(f, ":{keyword}"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
            Self::Callable(callable) => write!(f, "{callable}"),
        }
    }
}

impl PartialEq for NodeData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(lhs), Self::Number(rhs)) => {
                lhs.to_bits() == rhs.to_bits()
            }
            (lhs, rhs) => lhs.eq(rhs),
        }
    }
}

impl Eq for NodeData {}

impl std::hash::Hash for NodeData {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        match self {
            Self::Main(_) => state.write_u8(1),
            Self::Table(_) => state.write_u8(2),
            Self::PArray(_) => state.write_u8(3),
            Self::BArray(_) => state.write_u8(4),
            Self::Struct(_) => state.write_u8(5),
            Self::PTuple(_) => state.write_u8(6),
            Self::BTuple(_) => state.write_u8(7),
            Self::Number(_) => state.write_u8(8),
            Self::String(_) => state.write_u8(9),
            Self::Buffer(_) => state.write_u8(10),
            Self::Symbol(_) => state.write_u8(11),
            Self::Keyword(_) => state.write_u8(12),
            Self::True => state.write_u8(13),
            Self::False => state.write_u8(14),
            Self::Nil => state.write_u8(15),
            Self::Callable(_) => state.write_u8(16),
        }

        match self {
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
