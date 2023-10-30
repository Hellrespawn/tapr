mod arguments;
mod environment;
mod native;
mod value;

use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

pub use arguments::Arguments;
pub use native::{NativeFunction, NativeFunctionImpl};
pub use value::{Callable, CallableType, Value};

pub use self::environment::Environment;
use self::native::get_native_environment;
use self::value::Function;
use crate::error::{Error, ErrorKind};
use crate::location::Location;
use crate::parser::ast;
use crate::parser::ast::Node;
use crate::parser::ast::Special::If;
use crate::parser::parameters::Parameters;
use crate::visitor::Visitor;
use crate::Result;

pub struct Interpreter<'i> {
    pub output: Box<dyn Write + 'i>,
    environment: Environment,
}

impl<'i> Default for Interpreter<'i> {
    fn default() -> Self {
        let environment = get_native_environment();

        Self { output: Box::new(std::io::stdout()), environment }
    }
}

impl<'i> Interpreter<'i> {
    pub fn new(output: Box<dyn Write + 'i>, environment: Environment) -> Self {
        Self { output, environment }
    }

    pub fn interpret(&mut self, source: &str, name: &str) -> Result<Value> {
        let node = Node::from_string(source, name)?;

        node.accept(self)
    }

    pub fn push_environment(&mut self, new_environment: Environment) {
        let old_environment =
            std::mem::replace(&mut self.environment, new_environment);

        self.environment.set_parent(old_environment);
    }

    pub fn pop_environment(&mut self) -> Environment {
        let parent_environment =
            self.environment.take_parent().expect("Scope to have parent.");

        std::mem::replace(&mut self.environment, parent_environment)
    }

    fn get(&self, name: &str, location: Location) -> Result<Value> {
        if let Some(value) = self.environment.get(name) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                location,
                ErrorKind::SymbolNotDefined(name.to_owned()),
            ))
        }
    }

    fn enter_scope(&mut self) {
        let new_environment = Environment::new();

        let old_environment =
            std::mem::replace(&mut self.environment, new_environment);

        self.environment.set_parent(old_environment);
    }

    fn exit_scope(&mut self) -> Environment {
        self.pop_environment()
    }

    fn add_location_to_error(mut error: Error, location: Location) -> Error {
        error.location = error.location.or(Some(location));
        error
    }
}

impl<'i> Visitor<Result<Value>> for Interpreter<'i> {
    fn visit_node(&mut self, node: &Node) -> Result<Value> {
        match node.data() {
            ast::NodeData::Main(nodes) => self.visit_main(nodes),
            ast::NodeData::Special(special) => {
                match &**special {
                    If { condition, then, else_branch } => {
                        self.visit_if(condition, then, else_branch.as_ref())
                    },
                    ast::Special::Fn { parameters, body } => {
                        self.visit_fn(parameters, body)
                    },
                    ast::Special::Set { name, value } => {
                        self.visit_set(name, value, node.location())
                    },

                    ast::Special::Var { name, value } => {
                        self.visit_var(name, value, node.location())
                    },
                    ast::Special::Import { name, prefix } => {
                        self.visit_import(name, prefix.as_ref())
                    },
                }
            },
            ast::NodeData::List { literal, nodes } => {
                self.visit_list(*literal, nodes)
            },
            ast::NodeData::Symbol { module, value } => {
                self.visit_symbol(module.as_ref(), value, node.location())
            },
            ast::NodeData::Keyword(keyword) => {
                Ok(Value::Keyword(keyword.clone()))
            },
            ast::NodeData::Number(number) => Ok(Value::Number(*number)),
            ast::NodeData::String(string) => Ok(Value::String(string.clone())),
            ast::NodeData::True => Ok(Value::Boolean(true)),
            ast::NodeData::False => Ok(Value::Boolean(false)),
            ast::NodeData::Nil => Ok(Value::Nil),
        }
    }

    fn visit_main(&mut self, nodes: &[Node]) -> Result<Value> {
        let mut values =
            nodes.iter().map(|n| n.accept(self)).collect::<Result<Vec<_>>>()?;

        Ok(values.pop().unwrap_or(Value::Nil))
    }

    fn visit_fn(
        &mut self,
        parameters: &Parameters,
        body: &[Node],
    ) -> Result<Value> {
        let function = Function::new(parameters.clone(), body.to_vec());

        Ok(Value::Callable(Arc::new(function)))
    }

    fn visit_if(
        &mut self,
        condition: &Node,
        then: &Node,
        else_branch: Option<&Node>,
    ) -> Result<Value> {
        if condition.accept(self)?.is_truthy() {
            then.accept(self)
        } else if let Some(else_branch) = else_branch {
            else_branch.accept(self)
        } else {
            Ok(Value::Nil)
        }
    }

    fn visit_import(
        &mut self,
        name: &str,
        prefix: Option<&String>,
    ) -> Result<Value> {
        let path = {
            let path = PathBuf::from(name);
            if path.extension().is_some() {
                path
            } else {
                path.with_extension("tapr")
            }
        };

        let source = std::fs::read_to_string(&path)?;

        let prefix = prefix
        .cloned()
        .unwrap_or(path.file_stem().expect("std::fs::read_to_string should guarantee existence of file and file name.").to_string_lossy().to_string());

        // If prefix is empty, run everything in the current scope.
        if !prefix.is_empty() {
            // Do this to check that prefix isn't inserted yet.
            self.environment.insert(prefix.clone(), Value::Nil)?;
            self.enter_scope();
        }

        self.interpret(&source, name)?;

        if !prefix.is_empty() {
            let module_environment = self.exit_scope();

            let value = Value::Module(module_environment);

            self.environment.set(prefix, value).expect(
                "The module prefix should have already been initialized to Nil",
            );
        }

        Ok(Value::Nil)
    }

    fn visit_set(
        &mut self,
        name: &str,
        value: &Node,
        location: Location,
    ) -> Result<Value> {
        let value = value.accept(self)?;

        self.environment
            .set(name.to_owned(), value)
            .map_err(|e| Self::add_location_to_error(e, location))?;

        Ok(Value::Symbol(name.to_owned()))
    }

    fn visit_var(
        &mut self,
        name: &str,
        value: &Node,
        location: Location,
    ) -> Result<Value> {
        let value = value.accept(self)?;

        self.environment
            .insert(name.to_owned(), value)
            .map_err(|e| Self::add_location_to_error(e, location))?;

        Ok(Value::Symbol(name.to_owned()))
    }

    fn visit_list(&mut self, literal: bool, nodes: &[Node]) -> Result<Value> {
        {
            if literal {
                Ok(Value::List(
                    nodes
                        .iter()
                        .map(|n| n.accept(self))
                        .collect::<Result<Vec<_>>>()?,
                ))
            } else {
                if nodes.is_empty() {
                    return Ok(Value::Nil);
                }

                let node = &nodes[0];
                let value = nodes[0].accept(self)?;

                let Value::Callable(callable) = value else {
                    return Err(Error::new(
                        node.location(),
                        ErrorKind::NotCallable(value),
                    ));
                };

                let arguments_vec = nodes[1..]
                    .iter()
                    .map(|n| n.accept(self))
                    .collect::<Result<Vec<_>>>()?;

                let parameters = callable.parameters();

                let arguments = Arguments::new(&parameters, arguments_vec)
                    .map_err(|e| {
                        Self::add_location_to_error(e, node.location())
                    })?;

                callable.call(self, arguments).map_err(|e| {
                    Self::add_location_to_error(e, node.location())
                })
            }
        }
    }

    fn visit_symbol(
        &mut self,
        module: Option<&String>,
        value: &str,
        location: Location,
    ) -> Result<Value> {
        if let Some(module) = module {
            let Value::Module(environment) = self.get(module, location)? else {
                return Err(Error::new(
                    location,
                    ErrorKind::ModuleNotDefined(module.clone()),
                ));
            };

            self.push_environment(environment);

            let value = self.get(value, location)?;

            self.exit_scope();

            Ok(value)
        } else {
            self.get(value, location)
        }
    }

    // fn visit_while(&mut self, while_expr: &ast::While) -> Result<Value> {
    //     let ast::While {
    //         condition,
    //         expression,
    //     } = while_expr;

    //     let mut list = Vec::new();

    //     while condition.accept(self)?.is_truthy() {
    //         list.push(expression.accept(self)?);
    //     }

    //     Ok(Value::List(list))
    // }
}
