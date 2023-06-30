use self::native::get_default_environment;
use self::value::Function;
use crate::error::{Error, ErrorKind};
use crate::location::Location;
use crate::parser::parse_string;
use crate::{Node, NodeData, ParameterType, Result, Visitor};
use std::collections::HashMap;
use std::sync::Arc;

mod arguments;
mod environment;
mod native;
mod value;

pub use arguments::Arguments;
pub use environment::Environment;
pub use native::{NativeFunction, NativeFunctionImpl};
pub use value::{Callable, CallableType, Value};

#[derive(Debug, Copy, Clone)]
enum IntpMode {
    Default,
    Quote,
    Quasiquote,
    Unquote,
}

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
    mode: IntpMode,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            environment: get_default_environment(),
            mode: IntpMode::Default,
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            mode: IntpMode::Default,
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
            .collect::<Result<Vec<_>>>()
            .map_err(|e| Self::add_location_to_error(e, location))?
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
            .map_err(|e| Self::add_location_to_error(e, location))
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
            .collect::<Result<Vec<_>>>()
            .map_err(|e| Self::add_location_to_error(e, location))?;

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

        if let NodeData::Symbol(symbol) = nodes[0].data() {
            if symbol.is_special_form() {
                return self.visit_special_form(location, symbol, &nodes[1..]);
            }
        }

        let first_node = nodes[0].accept(self)?;

        match first_node {
            Value::Function(callable) => {
                self.visit_function(location, &*callable, &nodes[1..])
            }
            Value::Macro(callable) => {
                let node =
                    self.visit_macro(location, &*callable, &nodes[1..])?;

                node.accept(self)
            }
            _ => todo!("Throw error"),
        }
    }

    fn visit_function(
        &mut self,
        location: Location,
        callable: &dyn Callable<Value>,
        arguments: &[Node],
    ) -> Result<Value> {
        let arguments = arguments
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?;

        let parameters = callable.parameters();
        let arguments = Arguments::from_values(&parameters, arguments)?;

        callable
            .call(self, arguments)
            .map_err(|e| Self::add_location_to_error(e, location))
    }

    fn visit_macro(
        &mut self,
        location: Location,
        callable: &dyn Callable<Node>,
        arguments: &[Node],
    ) -> Result<Node> {
        todo!("Implement macro");
        // let node = callable
        //     .call(self, arguments.to_owned())
        //     .map_err(|e| Self::add_location_to_error(e, location))?;

        // node.accept(self)
    }

    fn visit_special_form(
        &mut self,
        location: Location,
        symbol: &str,
        arguments: &[Node],
    ) -> Result<Value> {
        let result = match symbol {
            "def" => self.visit_def(arguments),
            "var" => self.visit_var(arguments),
            "fn" => self.visit_fn(arguments),
            "do" => todo!(),
            "quote" => todo!(),
            "if" => self.visit_if(arguments),
            "splice" => todo!(),
            "while" => todo!(),
            "break" => todo!(),
            "set" => todo!(),
            "quasiquote" => todo!(),
            "unquote" => todo!(),
            "upscope" => todo!(),
            _ => unreachable!(),
        };

        result.map_err(|e| Self::add_location_to_error(e, location))
    }

    fn visit_def(&mut self, arguments: &[Node]) -> Result<Value> {
        let parameters = "k:symbol v".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        let key = arguments.unwrap_symbol(0);
        let value = arguments.unwrap(1).accept(self)?;

        self.environment.def(key.clone(), value)?;

        Ok(Value::Symbol(key))
    }

    fn visit_var(&mut self, arguments: &[Node]) -> Result<Value> {
        let parameters = "k:symbol v".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        let key = arguments.unwrap_symbol(0);
        let value = arguments.unwrap(1).accept(self)?;

        self.environment.var(key.clone(), value)?;

        Ok(Value::Symbol(key))
    }

    #[allow(clippy::unused_self)]
    fn visit_fn(&mut self, arguments: &[Node]) -> Result<Value> {
        let parameters = "l:list & body".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        let function = Function::new(
            arguments.parse_parameters(0)?,
            arguments.unwrap_from(1),
        );

        Ok(Value::Function(Arc::new(function)))
    }

    #[allow(clippy::unused_self)]
    fn visit_if(&mut self, arguments: &[Node]) -> Result<Value> {
        let parameters = "condition then &opt else".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        if arguments.unwrap(0).accept(self)?.is_truthy() {
            arguments.unwrap(1).accept(self)
        } else if let Some(node) = arguments.get(2) {
            node.accept(self)
        } else {
            Ok(Value::Nil)
        }
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

trait IsSpecialForm {
    fn is_special_form(&self) -> bool;
}

impl IsSpecialForm for String {
    fn is_special_form(&self) -> bool {
        matches!(
            self.as_str(),
            "def"
                | "var"
                | "fn"
                | "do"
                | "quote"
                | "if"
                | "splice"
                | "while"
                | "break"
                | "set"
                | "quasiquote"
                | "unquote"
                | "upscope"
        )
    }
}
