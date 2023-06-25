use std::collections::HashMap;

use super::reader_macro::ReaderMacro;
use crate::env::{DebugAst, DEBUG_AST, DEBUG_PARSER};
use crate::graph::GraphVisitor;
use crate::location::Location;
use crate::parser::parameters::Parameters;
use crate::parser::{Parser, Rule};
use crate::visitor::Visitor;
use crate::Result;
use itertools::Itertools;
use pest::iterators::{Pair, Pairs};
use pest::Parser as PestParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    reader_macros: Vec<ReaderMacro>,
    location: Location,
    data: NodeData,
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

impl Node {
    pub fn mock(data: NodeData) -> Self {
        Self {
            reader_macros: vec![],
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

    pub fn from_string(source: &str, name: &str) -> Result<Node> {
        let mut pairs = Parser::parse(Rule::main, source)?;

        if *DEBUG_PARSER {
            println!("{pairs:#?}");
        }

        let node = Node::parse(
            pairs.next().expect("Pairs<Rule::main> panicked on next()"),
        );

        if !matches!(*DEBUG_AST, DebugAst::Off) {
            GraphVisitor::create_ast_graph(
                &node,
                name,
                matches!(*DEBUG_AST, DebugAst::RetainDot),
            );
        }

        Ok(node)
    }

    fn parse(pair: Pair<Rule>) -> Node {
        let location = Location::from_pair(&pair);

        let data = match pair.as_rule() {
            Rule::symbol => NodeData::Symbol(
                pair.into_inner()
                    .next()
                    .expect("Rule::symbol should have Rule::token")
                    .as_str()
                    .to_owned(),
            ),
            Rule::keyword => NodeData::Keyword(
                pair.as_str().get(1..).unwrap_or("").to_owned(),
            ),
            Rule::constant => match pair.as_str() {
                "true" => NodeData::True,
                "false" => NodeData::False,
                "nil" => NodeData::Nil,
                other => unreachable!("Invalid constant '{other}'"),
            },
            Rule::string | Rule::long_string => {
                NodeData::String(Self::extract_string(pair))
            }
            Rule::buffer | Rule::long_buffer => {
                NodeData::Buffer(Self::extract_string(pair))
            }
            Rule::number => {
                NodeData::Number(pair.as_str().parse().unwrap_or_else(|_| {
                    panic!("Unable to parse '{}' as number", pair.as_str())
                }))
            }
            Rule::value => return Self::parse_value(pair),
            Rule::p_tuple => {
                NodeData::PTuple(pair.into_inner().map(Node::parse).collect())
            }
            Rule::b_tuple => {
                NodeData::BTuple(pair.into_inner().map(Node::parse).collect())
            }
            Rule::struct_ => NodeData::Struct(
                pair.into_inner().map(Node::parse).tuples().collect(),
            ),
            Rule::p_array => {
                NodeData::PArray(pair.into_inner().map(Node::parse).collect())
            }
            Rule::b_array => {
                NodeData::BArray(pair.into_inner().map(Node::parse).collect())
            }
            Rule::table => NodeData::Table(
                pair.into_inner().map(Node::parse).tuples().collect(),
            ),
            Rule::main => NodeData::Main(
                pair.into_inner()
                    .take_while(|p| p.as_rule() != Rule::EOI)
                    .map(Node::parse)
                    .collect(),
            ),
            rule => {
                unreachable!("Attempted to parse '{:?}':\n'{:#?}'", rule, pair,)
            }
        };

        Node {
            reader_macros: vec![],
            location,
            data,
        }
    }

    fn parse_value(pair: Pair<Rule>) -> Node {
        let location = Location::from_pair(&pair);

        let mut pairs = pair.into_inner().collect::<Vec<_>>();

        let pair = pairs
            .pop()
            .expect("Rule::value should always have a raw_value");

        let reader_macros = ReaderMacro::from_pairs(pairs);

        let mut node = Node::parse(pair);

        node.reader_macros = reader_macros;
        node.location = location;

        node
    }

    fn extract_string(pair: Pair<Rule>) -> String {
        pair.into_inner()
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .to_owned()
    }
}

pub fn parse_parameters(pairs: Pairs<Rule>) -> Parameters {
    todo!()
    // let parameters = pairs
    //     .flat_map(|p| match p.as_rule() {
    //         Rule::parameters => p
    //             .into_inner()
    //             .map(|p| parameter(p, false))
    //             .collect::<Vec<_>>(),
    //         Rule::optional_parameters => optional_parameters(p),
    //         Rule::rest_parameter => vec![rest_parameter(p)],
    //         other => unreachable!("{:?}", other),
    //     })
    //     .collect();

    // Parameters::new(parameters)
    //     .expect("Grammar did not parse valid parameters for function.")
}

// fn parameter(pair: Pair<Rule>, optional: bool) -> Parameter {
//     let mut inner = pair.into_inner();

//     let name = inner.next().unwrap().as_str().to_owned();
//     let ptypes = inner
//         .map(|p| match p.as_str() {
//             "bool" => ParameterType::Boolean,
//             "number" => ParameterType::Number,
//             "string" => ParameterType::String,
//             "list" => ParameterType::List,
//             "module" => ParameterType::Module,
//             "function" => ParameterType::Function,
//             "nil" => ParameterType::Nil,
//             other => unreachable!("{:?}", other),
//         })
//         .collect();

//     Parameter::new(name, ptypes, optional, false)
// }

// fn optional_parameters(pair: Pair<Rule>) -> Vec<Parameter> {
//     pair.into_inner()
//         .map(|p| parameter(p, true))
//         .collect::<Vec<_>>()
// }

// fn rest_parameter(pair: Pair<Rule>) -> Parameter {
//     pair.into_inner()
//         .map(|p| parameter(p, false))
//         .next()
//         .unwrap()
//         .rest()
// }
