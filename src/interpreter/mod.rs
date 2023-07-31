use crate::callable::{get_default_environment, Function};
use crate::error::{Error, ErrorKind};
use crate::location::Location;
use crate::node::NodeSource;
use crate::parser::parse_string;
use crate::{Node, NodeData, Result, Visitor};
use std::collections::HashMap;
use std::sync::Arc;

mod environment;

pub use crate::arguments::Arguments;
pub use crate::{Callable, CallableType};
pub use environment::Environment;

#[derive(Debug, Copy, Clone)]
enum IntpMode {
    Default,
    Quote,
    Quasiquote,
    Unquote,
}

impl IntpMode {
    fn is_quoted(self) -> bool {
        matches!(self, Self::Quote | Self::Quasiquote)
    }
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

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    pub fn interpret(&mut self, source: &str, name: &str) -> Result<Node> {
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

    fn enter_scope(&mut self) {
        self.push_environment(Environment::new());
    }

    fn exit_scope(&mut self) {
        self.pop_environment();
    }

    fn add_location_to_error(mut error: Error, source: NodeSource) -> Error {
        error.location = error.location.or(source.location());
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
        source: NodeSource,
        nodes: &[Node],
    ) -> Result<Node> {
        Ok(nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()
            .map_err(|e| Self::add_location_to_error(e, source))?
            .pop()
            .unwrap_or(Node::new(source, NodeData::Nil)))
    }

    fn visit_table(
        &mut self,
        source: NodeSource,
        map: &HashMap<Node, Node>,
    ) -> Result<Node> {
        let map = self.visit_map(source, map)?;

        Ok(Node::new(source, NodeData::Table(map)))
    }

    fn visit_struct(
        &mut self,
        source: NodeSource,
        map: &HashMap<Node, Node>,
    ) -> Result<Node> {
        let map = self.visit_map(source, map)?;

        Ok(Node::new(source, NodeData::Struct(map)))
    }

    fn visit_map(
        &mut self,
        source: NodeSource,
        map: &HashMap<Node, Node>,
    ) -> Result<HashMap<Node, Node>> {
        map.iter()
            .map(|(k, v)| self.visit_key_value(k, v))
            .collect::<Result<HashMap<_, _>>>()
            .map_err(|e| Self::add_location_to_error(e, source))
    }

    fn visit_p_array(
        &mut self,
        source: NodeSource,
        nodes: &[Node],
    ) -> Result<Node> {
        let values = self.visit_list(source, nodes)?;

        Ok(Node::new(source, NodeData::PArray(values)))
    }

    fn visit_b_array(
        &mut self,
        source: NodeSource,
        nodes: &[Node],
    ) -> Result<Node> {
        let values = self.visit_list(source, nodes)?;

        Ok(Node::new(source, NodeData::BArray(values)))
    }

    fn visit_b_tuple(
        &mut self,
        source: NodeSource,
        nodes: &[Node],
    ) -> Result<Node> {
        let values = self.visit_list(source, nodes)?;

        Ok(Node::new(source, NodeData::BTuple(values)))
    }

    fn visit_list(
        &mut self,
        source: NodeSource,
        nodes: &[Node],
    ) -> Result<Vec<Node>> {
        let visited_nodes = nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()
            .map_err(|e| Self::add_location_to_error(e, source))?;

        Ok(visited_nodes)
    }

    fn visit_p_tuple(
        &mut self,
        source: NodeSource,
        nodes: &[Node],
    ) -> Result<Node> {
        if nodes.is_empty() {
            Ok(Node::new(source, NodeData::Nil))
        } else if self.mode.is_quoted() && !nodes[0].is_unquote() {
            Ok(Node::new(
                source,
                NodeData::PTuple(self.visit_list(source, nodes)?),
            ))
        } else {
            if let NodeData::Symbol(symbol) = &nodes[0].data() {
                if symbol.is_special_form() {
                    return self.visit_special_form(
                        source,
                        symbol,
                        &nodes[1..],
                    );
                }
            }

            let first_node = nodes[0].accept(self)?;

            match first_node {
                Node::Function(callable) => {
                    let arguments = nodes[1..]
                        .iter()
                        .map(|n| n.accept(self))
                        .collect::<Result<Vec<_>>>()?;

                    self.visit_function(source, &*callable, &arguments)
                }
                Node::Macro(callable) => {
                    let node =
                        self.visit_macro(source, &*callable, &nodes[1..])?;

                    node.accept(self)
                }
                other => Err(ErrorKind::NotCallable(other).into()),
            }
        }
    }

    fn visit_function(
        &mut self,
        source: NodeSource,
        callable: &dyn Callable,
        arguments: &[Node],
    ) -> Result<Node> {
        let parameters = callable.parameters();
        let arguments =
            Arguments::from_values(&parameters, arguments.to_owned())?;

        callable
            .call(source, self, arguments)
            .map_err(|e| Self::add_location_to_error(e, location))
    }

    fn visit_macro(
        &mut self,
        source: NodeSource,
        callable: &dyn Callable,
        arguments: &[Node],
    ) -> Result<Node> {
        let parameters = callable.parameters();
        let arguments =
            Arguments::from_nodes(&parameters, arguments.to_owned())?;

        callable
            .call(source, self, arguments)
            .map_err(|e| Self::add_location_to_error(e, location))
            .map_err(|e| Self::add_location_to_error(e, location))
    }

    fn visit_special_form(
        &mut self,
        source: NodeSource,
        symbol: &str,
        arguments: &[Node],
    ) -> Result<Node> {
        #[allow(clippy::match_same_arms)]
        let result = match symbol {
            "def" => self.visit_def(arguments),
            "var" => self.visit_var(arguments),
            "fn" => self.visit_fn(arguments),
            "do" => self.visit_do(arguments),
            "quote" => self.visit_quote(arguments),
            "if" => self.visit_if(arguments),
            "splice" => self.visit_noop(source, arguments),
            "while" => self.visit_while(arguments),
            "break" => self.visit_noop(source, arguments),
            "set" => self.visit_noop(source, arguments),
            "quasiquote" => self.visit_quasiquote(arguments),
            "unquote" => self.visit_unquote(arguments),
            "upscope" => self.visit_noop(source, arguments),
            _ => unreachable!(),
        };

        result.map_err(|e| Self::add_location_to_error(e, location))
    }

    fn visit_def(&mut self, arguments: &[Node]) -> Result<Node> {
        let parameters = "k:symbol v".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        let key = arguments.unwrap_symbol(0);
        let value = arguments.unwrap(1).accept(self)?;

        self.environment.def(key.clone(), value)?;

        Ok(Node::symbol(key))
    }

    fn visit_var(&mut self, arguments: &[Node]) -> Result<Node> {
        let parameters = "k:symbol v".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        let key = arguments.unwrap_symbol(0);
        let value = arguments.unwrap(1).accept(self)?;

        self.environment.var(key.clone(), value)?;

        Ok(Node::symbol(key))
    }

    #[allow(clippy::unused_self)]
    fn visit_fn(&mut self, arguments: &[Node]) -> Result<Node> {
        let parameters = "l:list & body".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        let function = Function::new(
            arguments.parse_parameters(0)?,
            arguments.unwrap_from(1),
        );

        Ok(Node::Function(Arc::new(function)))
    }

    #[allow(clippy::unused_self)]
    fn visit_do(&mut self, arguments: &[Node]) -> Result<Node> {
        self.enter_scope();

        let result = arguments
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>();

        self.exit_scope();

        Ok(result?.pop().unwrap_or(Node::nil()))
    }

    fn visit_quote(&mut self, arguments: &[Node]) -> Result<Node> {
        self.visit_with_mode(IntpMode::Quote, arguments)
    }

    fn visit_if(&mut self, arguments: &[Node]) -> Result<Node> {
        let parameters = "condition then &opt else".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        if arguments.unwrap(0).accept(self)?.is_truthy() {
            arguments.unwrap(1).accept(self)
        } else if let Some(node) = arguments.get(2) {
            node.accept(self)
        } else {
            Ok(Node::nil())
        }
    }

    fn visit_while(&mut self, arguments: &[Node]) -> Result<Node> {
        let parameters = "condition & body".try_into().unwrap();

        let arguments = Arguments::from_nodes(&parameters, arguments.to_vec())?;

        let condition = arguments.unwrap(0);
        let body = arguments.unwrap_from(1);

        while condition.accept(self)?.is_truthy() {
            for node in &body {
                node.accept(self)?;
            }
        }

        Ok(Node::nil())
    }

    fn visit_quasiquote(&mut self, arguments: &[Node]) -> Result<Node> {
        self.visit_with_mode(IntpMode::Quasiquote, arguments)
    }

    fn visit_unquote(&mut self, arguments: &[Node]) -> Result<Node> {
        if matches!(self.mode, IntpMode::Quasiquote) {
            self.visit_with_mode(IntpMode::Unquote, arguments)
        } else {
            Err(ErrorKind::Message(
                "unquote is only valid inside quasiquote.".to_owned(),
            )
            .into())
        }
    }

    fn visit_symbol(
        &mut self,
        source: NodeSource,
        symbol: &str,
    ) -> Result<Node> {
        if self.mode.is_quoted() {
            Ok(Node::symbol(symbol.to_owned()))
        } else if let Some(value) = self.environment.get(symbol) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                location,
                ErrorKind::SymbolNotDefined(symbol.to_owned()),
            ))
        }
    }

    fn visit_noop(
        &mut self,
        _source: NodeSource,
        _arguments: &[Node],
    ) -> Result<Node> {
        Ok(Node::nil())
    }

    fn visit_with_mode(
        &mut self,
        new_mode: IntpMode,
        arguments: &[Node],
    ) -> Result<Node> {
        let parameters = "x".try_into().unwrap();

        let argument =
            Arguments::from_nodes(&parameters, arguments.to_vec())?.unwrap(0);

        let old_mode = self.mode;
        self.mode = new_mode;

        let result = argument.accept(self);

        self.mode = old_mode;

        result
    }
}

impl Visitor<Result<Node>> for Interpreter {
    fn visit_node(&mut self, node: &Node) -> Result<Node> {
        let source = node.source();

        match node.data() {
            NodeData::Main(nodes) => self.visit_main(source, nodes),
            NodeData::Table(map) => self.visit_table(source, map),
            NodeData::PArray(nodes) => self.visit_p_array(source, nodes),
            NodeData::BArray(nodes) => self.visit_b_array(source, nodes),
            NodeData::Struct(map) => self.visit_struct(source, map),
            NodeData::PTuple(nodes) => self.visit_p_tuple(source, nodes),
            NodeData::BTuple(nodes) => self.visit_b_tuple(source, nodes),
            NodeData::Symbol(symbol) => self.visit_symbol(source, symbol),
            other => Ok(node.clone()),
            // NodeData::Number(number) => Ok(Node::number(*number)),
            // NodeData::String(string) => Ok(Node::string(string.clone())),
            // NodeData::Buffer(string) => Ok(Node::buffer(string.clone())),
            // NodeData::Keyword(keyword) => Ok(Node::keyword(keyword.clone())),
            // NodeData::True => Ok(Node::bool(true)),
            // NodeData::False => Ok(Node::bool(false)),
            // NodeData::Nil => Ok(Node::nil()),
            _ => todo!("Handle visit for intp-only nodes"),
        }
        .map_err(|e| Self::add_location_to_error(e, source))
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
