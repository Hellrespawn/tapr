use crate::location::Location;
use crate::visitor::Visitor;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    location: Location,
    data: NodeData<Node>,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data())
    }
}

impl Node {
    pub fn new(location: Location, data: NodeData<Node>) -> Self {
        Self { location, data }
    }

    pub fn mock(data: NodeData<Node>) -> Self {
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

    pub fn data(&self) -> &NodeData<Node> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut NodeData<Node> {
        &mut self.data
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn is_unquote(&self) -> bool {
        if let NodeData::Symbol(symbol) = self.data() {
            if symbol == "unquote" {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone)]
pub enum NodeData<T> {
    Main(Vec<T>),
    Table(HashMap<T, T>),
    PArray(Vec<T>),
    BArray(Vec<T>),
    Struct(HashMap<T, T>),
    PTuple(Vec<T>),
    BTuple(Vec<T>),
    Number(f64),
    String(String),
    Buffer(String),
    Symbol(String),
    Keyword(String),
    True,
    False,
    Nil,
}

impl<T> std::fmt::Display for NodeData<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl<T: PartialEq + Eq + std::hash::Hash> PartialEq for NodeData<T> {
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

            (NodeData::True, NodeData::True)
            | (NodeData::False, NodeData::False)
            | (NodeData::Nil, NodeData::Nil) => true,
            (NodeData::Number(lhs), NodeData::Number(rhs)) => {
                lhs.to_bits() == rhs.to_bits()
            }
            _ => false,
        }
    }
}

impl<T: PartialEq + Eq + std::hash::Hash> Eq for NodeData<T> {}

impl<T: PartialOrd + PartialEq + Eq + std::hash::Hash> PartialOrd
    for NodeData<T>
{
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

impl<T: std::hash::Hash> std::hash::Hash for NodeData<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        core::mem::discriminant(self).hash(state);

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
