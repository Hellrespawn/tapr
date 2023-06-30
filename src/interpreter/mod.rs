use self::value::{Callable, CallableType};
use crate::error::{Error, ErrorKind};
use crate::location::Location;
use crate::parser::parse_string;
use crate::{Node, NodeData, Result, Visitor};
use arguments::Arguments;
use environment::Environment;
use std::collections::HashMap;

pub use value::Value;

mod arguments;
mod environment;
mod native;
mod value;

#[derive(Debug, Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, source: &str, name: &str) -> Result<Value> {
        let node = parse_string(source, name)?;

        node.accept(self)
    }

    pub fn push_environment(&mut self, new_environment: Environment) {
        let old_environment =
            std::mem::replace(&mut self.environment, new_environment);

        self.environment.set_parent(old_environment);
    }

    pub fn pop_environment(&mut self) -> Environment {
        let parent_environment = self
            .environment
            .take_parent()
            .expect("Scope to have parent.");

        std::mem::replace(&mut self.environment, parent_environment)
    }

    fn add_location_to_error(mut error: Error, location: Location) -> Error {
        error.location = error.location.or(Some(location));
        error
    }

    fn visit_key_value(
        &mut self,
        key: &Node,
        value: &Node,
    ) -> Result<(Value, Value)> {
        Ok((key.accept(self)?, value.accept(self)?))
    }

    fn visit_main(
        &mut self,
        location: Location,
        nodes: &[Node],
    ) -> Result<Value> {
        Ok(nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?
            .pop()
            .unwrap_or(Value::Nil))
    }

    fn visit_map(
        &mut self,
        location: Location,
        map: &HashMap<Node, Node>,
        mutable: bool,
    ) -> Result<Value> {
        map.iter()
            .map(|(k, v)| self.visit_key_value(k, v))
            .collect::<Result<HashMap<_, _>>>()
            .map(|map| Value::Map { mutable, map })
    }

    fn visit_list(
        &mut self,
        location: Location,
        nodes: &[Node],
        mutable: bool,
        bracket: bool,
    ) -> Result<Value> {
        let visited_nodes = nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?;

        if !mutable && !bracket {
            unreachable!("Function calls should be handled separately.")
        }

        let value = Value::List {
            mutable,
            list: visited_nodes,
        };

        Ok(value)
    }

    fn visit_call(
        &mut self,
        location: Location,
        nodes: &[Node],
    ) -> Result<Value> {
        if nodes.is_empty() {
            return Ok(Value::Nil);
        }

        let first_node = nodes[0].accept(self)?;

        todo!("Handle Function Call")

        // if let Value::Callable(callable) = first_node {
        //     match callable.callable_type() {
        //         CallableType::Native | CallableType::Function => {
        //             self.visit_function(location, &*callable, &nodes[1..])
        //         }
        //         CallableType::Macro => {
        //             self.visit_macro(location, &*callable, &nodes[1..])
        //         }
        //         CallableType::SpecialForm => todo!(),
        //     }
        // } else {
        //     todo!("Throw error")
        // }
    }

    fn visit_function(
        &mut self,
        location: Location,
        callable: &dyn Callable<Result<Value>>,
        arguments: &[Node],
    ) -> Result<Value> {
        let arguments = arguments
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?;

        let arguments = Arguments::new(&callable.parameters(), arguments)?;

        callable
            .call(self, arguments)
            .map_err(|e| Self::add_location_to_error(e, location))
    }

    fn visit_macro(
        &mut self,
        location: Location,
        callable: &dyn Callable<Result<Node>>,
        arguments: &[Node],
    ) -> Result<Node> {
        todo!("Implement macro");
        // let node = callable
        //     .call(self, arguments.to_owned())
        //     .map_err(|e| Self::add_location_to_error(e, location))?;

        // node.accept(self)
    }

    fn visit_symbol(
        &mut self,
        location: Location,
        symbol: &str,
    ) -> Result<Value> {
        if let Some(value) = self.environment.get(symbol) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                location,
                ErrorKind::SymbolNotDefined(symbol.to_owned()),
            ))
        }
    }
}

impl Visitor<Result<Value>> for Interpreter {
    fn visit_node(&mut self, node: &Node) -> Result<Value> {
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
            NodeData::Number(number) => Ok(Value::Number(*number)),
            NodeData::String(string) => Ok(Value::string(string.clone())),
            NodeData::Buffer(string) => Ok(Value::String {
                mutable: true,
                string: string.clone(),
            }),
            NodeData::Keyword(keyword) => Ok(Value::Keyword(keyword.clone())),
            NodeData::True => Ok(Value::Boolean(true)),
            NodeData::False => Ok(Value::Boolean(false)),
            NodeData::Nil => Ok(Value::Nil),
        }
        .map_err(|e| Self::add_location_to_error(e, location))
    }
}
