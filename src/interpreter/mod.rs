mod arguments;
mod builtins;
mod environment;
mod parameters;
mod value;

pub use arguments::Arguments;
pub use parameters::{Parameter, ParameterType, Parameters};
pub use value::Value;

use self::environment::Environment;
use self::value::Lambda;
use crate::error::{Error, ErrorKind};
use crate::graph::GraphVisitor;
use crate::location::Location;
use crate::parser::ast::Node;
use crate::parser::ast::Special::If;
use crate::parser::{ast, DEBUG_AST};
use crate::visitor::Visitor;
use crate::Result;
use std::io::Write;

pub struct Interpreter<'i> {
    pub output: Box<dyn Write + 'i>,
    environment: Environment,
}

impl<'i> Default for Interpreter<'i> {
    fn default() -> Self {
        Self {
            output: Box::new(std::io::stdout()),
            environment: Environment::root(),
        }
    }
}

impl<'i> Interpreter<'i> {
    pub fn new(output: Box<dyn Write + 'i>, environment: Environment) -> Self {
        Self {
            output,
            environment,
        }
    }

    pub fn interpret(&mut self, source: &str, name: &str) -> Result<Value> {
        let node = Node::from_string(source)?;

        if *DEBUG_AST {
            GraphVisitor::create_ast_graph(&node, name);
        }

        node.accept(self)
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
        let new_environment = Environment::empty();

        let old_environment =
            std::mem::replace(&mut self.environment, new_environment);

        self.environment.set_parent(old_environment);
    }

    fn exit_scope(&mut self) {
        let parent_environment = self
            .environment
            .take_parent()
            .expect("Scope to have parent.");

        self.environment = parent_environment;
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
            ast::NodeData::Special(special) => match &**special {
                If {
                    condition,
                    then,
                    else_branch,
                } => self.visit_if(condition, then, else_branch.as_ref()),
                ast::Special::Fn { parameters, body } => {
                    self.visit_fn(parameters, body)
                }
                ast::Special::Set { name, value } => {
                    self.visit_set(name, value, node.location())
                }

                ast::Special::Var { name, value } => {
                    self.visit_var(name, value, node.location())
                }
            },
            ast::NodeData::List { literal, nodes } => {
                self.visit_list(*literal, nodes)
            }
            ast::NodeData::Symbol { module, value } => todo!(),
            ast::NodeData::Keyword(_) => todo!(),
            ast::NodeData::Number(_) => todo!(),
            ast::NodeData::String(_) => todo!(),
            ast::NodeData::True => todo!(),
            ast::NodeData::False => todo!(),
            ast::NodeData::Nil => todo!(),
        }
    }

    fn visit_main(&mut self, nodes: &[Node]) -> Result<Value> {
        let mut values = nodes
            .iter()
            .map(|n| n.accept(self))
            .collect::<Result<Vec<_>>>()?;

        Ok(values.pop().unwrap_or(Value::Nil))
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

    fn visit_fn(
        &mut self,
        parameters: &[String],
        body: &[Node],
    ) -> Result<Value> {
        let parameters = Parameters::new(
            parameters.iter().map(|s| Parameter::any(s)).collect(),
        )?;

        Ok(Value::Lambda(Lambda::new(parameters, body.to_vec())))
    }

    fn visit_set(
        &mut self,
        name: &str,
        value: &Node,
        location: Location,
    ) -> Result<Value> {
        let value = value.accept(self)?;

        if !self.environment.has_in_scope(name) {
            return Err(Error::new(
                location,
                ErrorKind::SymbolNotDefined(name.to_owned()),
            ));
        }

        self.environment.insert(name.to_owned(), value);

        Ok(Value::Symbol(name.to_owned()))
    }

    fn visit_var(
        &mut self,
        name: &str,
        value: &Node,
        location: Location,
    ) -> Result<Value> {
        let value = value.accept(self)?;

        if self.environment.has_in_scope(name) {
            return Err(Error::new(
                location,
                ErrorKind::SymbolDefined(name.to_owned()),
            ));
        }

        self.environment.insert(name.to_owned(), value);

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

                let symbol = nodes[0].accept(self)?;

                todo!()

                // let value = self.get(&call.symbol.0)?;

                // let arguments = call
                //     .arguments
                //     .iter()
                //     .map(|e| e.accept(self))
                //     .collect::<Result<Vec<_>>>()?;

                // match value {
                //     Value::Builtin(builtin) => builtin
                //         .call(self, arguments)
                //         .map_err(|e| Self::add_location_to_error(e, location)),
                //     Value::Lambda(lambda) => lambda.call(self, arguments),
                //     _ => {
                //         Err(Error::new(location, ErrorKind::NotCallable(value)))
                //     }
                // }
            }
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

    // fn visit_call(&mut self, call: &ast::Call) -> Result<Value> {
    //     let location = call.symbol.0.location;

    //     let value = self.get(&call.symbol.0)?;

    //     let arguments = call
    //         .arguments
    //         .iter()
    //         .map(|e| e.accept(self))
    //         .collect::<Result<Vec<_>>>()?;

    //     match value {
    //         Value::Builtin(builtin) => builtin
    //             .call(self, arguments)
    //             .map_err(|e| Self::add_location_to_error(e, location)),
    //         Value::Lambda(lambda) => lambda.call(self, arguments),
    //         _ => Err(Error::new(location, ErrorKind::NotCallable(value))),
    //     }
    // }

    // fn visit_quoted_datum(&mut self, datum: &ast::Datum) -> Result<Value> {
    //     // Quoted list is handled on the parser level.
    //     let value = match datum {
    //         ast::Datum::Symbol(symbol) => {
    //             Value::Symbol(symbol.0.lexeme().to_owned())
    //         }
    //         datum => self.visit_datum(datum)?,
    //     };

    //     Ok(value)
    // }

    // fn visit_datum(&mut self, datum: &ast::Datum) -> Result<Value> {
    //     let value = match datum {
    //         ast::Datum::List(list) => {
    //             let list = list
    //                 .expressions
    //                 .iter()
    //                 .map(|e| e.accept(self))
    //                 .collect::<Result<Vec<_>>>()?;

    //             Value::List(list)
    //         }
    //         ast::Datum::Boolean(boolean) => Value::Boolean(
    //             boolean
    //                 .0
    //                 .lexeme()
    //                 .parse()
    //                 .expect("Lexer should have checked boolean validity."),
    //         ),
    //         ast::Datum::Number(number) => Value::Number(
    //             number
    //                 .0
    //                 .lexeme()
    //                 .parse()
    //                 .expect("Lexer should have checked number validity."),
    //         ),
    //         ast::Datum::String(string) => {
    //             Value::String(string.0.lexeme().to_owned())
    //         }
    //         ast::Datum::Symbol(symbol) => self.get(&symbol.0)?,
    //         ast::Datum::Nil => Value::Nil,
    //     };

    //     Ok(value)
    // }
}
