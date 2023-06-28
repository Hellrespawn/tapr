use crate::error::{Error, ErrorKind};
use crate::location::Location;
use crate::node::callable::{Callable, CallableType};
use crate::parser::parse_string;
use crate::{Node, NodeData, Result, Visitor};
use environment::Environment;
use std::collections::HashMap;

mod environment;

#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, source: &str, name: &str) -> Result<Node> {
        let node = parse_string(source, name)?;

        node.accept(self)
    }

    fn add_location_to_error(mut error: Error, location: Location) -> Error {
        error.location = error.location.or(Some(location));
        error
    }

    fn visit_key_value(
        &mut self,
        key: &Node,
        value: &Node,
    ) -> Result<(Node, Node)> {
        Ok((key.accept(self)?, value.accept(self)?))
    }

    fn visit_main(
        &mut self,
        location: Location,
        nodes: &[Node],
    ) -> Result<Node> {
        Ok(nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?
            .pop()
            .unwrap_or(Node::new(location, NodeData::Nil)))
    }

    fn visit_map(
        &mut self,
        location: Location,
        map: &HashMap<Node, Node>,
        mutable: bool,
    ) -> Result<Node> {
        map.iter()
            .map(|(k, v)| self.visit_key_value(k, v))
            .collect::<Result<HashMap<_, _>>>()
            .map(|map| {
                Node::new(
                    location,
                    if mutable {
                        NodeData::Table(map)
                    } else {
                        NodeData::Struct(map)
                    },
                )
            })
    }

    fn visit_list(
        &mut self,
        location: Location,
        nodes: &[Node],
        mutable: bool,
        bracket: bool,
    ) -> Result<Node> {
        let visited_nodes = nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?;

        let data = match (mutable, bracket) {
            (true, true) => NodeData::BArray(visited_nodes),
            (true, false) => NodeData::PArray(visited_nodes),
            (false, true) => NodeData::BTuple(visited_nodes),
            (false, false) => {
                unreachable!("Function calls should be handled separately.")
            }
        };

        Ok(Node::new(location, data))
    }

    fn visit_call(
        &mut self,
        location: Location,
        nodes: &[Node],
    ) -> Result<Node> {
        if nodes.is_empty() {
            return Ok(Node::new(location, NodeData::Nil));
        }

        let first_node = nodes[0].accept(self)?;

        match first_node.as_callable() {
            Some(callable) => match callable.callable_type() {
                CallableType::Native | CallableType::Function => {
                    self.visit_function(location, callable, &nodes[1..])
                }
                CallableType::Macro => {
                    self.visit_macro(location, callable, &nodes[1..])
                }
                CallableType::SpecialForm => todo!(),
            },
            None => todo!("Throw error"),
        }
    }

    fn visit_function(
        &mut self,
        location: Location,
        callable: &dyn Callable,
        arguments: &[Node],
    ) -> Result<Node> {
        let arguments = arguments
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?;

        callable
            .call(self, arguments)
            .map_err(|e| Self::add_location_to_error(e, location))
    }

    fn visit_macro(
        &mut self,
        location: Location,
        callable: &dyn Callable,
        arguments: &[Node],
    ) -> Result<Node> {
        let node = callable
            .call(self, arguments.to_owned())
            .map_err(|e| Self::add_location_to_error(e, location))?;

        node.accept(self)
    }

    fn visit_symbol(
        &mut self,
        location: Location,
        symbol: &str,
    ) -> Result<Node> {
        if let Some(node) = self.environment.get(symbol) {
            Ok(node.clone())
        } else {
            Err(Error::new(
                location,
                ErrorKind::SymbolNotDefined(symbol.to_owned()),
            ))
        }
    }
}

impl Visitor<Result<Node>> for Interpreter {
    fn visit_node(&mut self, node: &Node) -> Result<Node> {
        let location = node.location();

        match node.data() {
            NodeData::Main(nodes) => self.visit_main(location, nodes),
            NodeData::Table(map) => self.visit_map(location, map, true),
            NodeData::PArray(nodes) => {
                self.visit_list(location, nodes, true, false)
            }
            NodeData::BArray(nodes) => {
                self.visit_list(location, nodes, true, true)
            }
            NodeData::Struct(map) => self.visit_map(location, map, false),
            NodeData::PTuple(nodes) => self.visit_call(location, nodes),
            NodeData::BTuple(nodes) => {
                self.visit_list(location, nodes, false, true)
            }
            NodeData::Symbol(symbol) => self.visit_symbol(location, symbol),
            NodeData::Number(_)
            | NodeData::String(_)
            | NodeData::Buffer(_)
            | NodeData::Keyword(_)
            | NodeData::True
            | NodeData::False
            | NodeData::Nil => Ok(node.clone()),
            _ => unreachable!(),
        }
        .map_err(|e| Self::add_location_to_error(e, location))
    }
}
