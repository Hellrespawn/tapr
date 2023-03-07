mod builtin;
mod environment;
mod value;

pub mod callable;
pub mod parameters;

pub use value::Value;

use self::environment::Environment;
use self::parameters::{Parameter, ParameterType, Parameters};
use self::value::Function;
use crate::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::parser::ast::*;
use crate::parser::Parser;
use crate::token::Token;
use crate::visitor::Visitor;
use crate::Result;
use callable::Callable;
use std::io::Write;
use std::rc::Rc;

pub struct Interpreter<'i> {
    pub output: Box<dyn Write + 'i>,
    environment: Environment,
    parser_no: usize,
    repl: bool,
}

impl<'i> Default for Interpreter<'i> {
    fn default() -> Self {
        Self {
            output: Box::new(std::io::stdout()),
            environment: Environment::root(),
            parser_no: 1,
            repl: false,
        }
    }
}

impl<'i> Interpreter<'i> {
    pub fn new(
        output: Box<dyn Write + 'i>,
        environment: Environment,
        parser_no: usize,
        repl: bool,
    ) -> Self {
        Self {
            output,
            environment,
            parser_no,
            repl,
        }
    }

    pub fn repl() -> Self {
        Self::new(Box::new(std::io::stdout()), Environment::root(), 1, true)
    }

    pub fn with_parser_no(parser_no: usize) -> Self {
        Self::new(
            Box::new(std::io::stdout()),
            Environment::root(),
            parser_no,
            false,
        )
    }

    pub fn interpret(&mut self, source: &str) -> Result<Value> {
        let lexer = Lexer::new(source);

        let mut parser = Parser::new(lexer);

        let result = parser.parse().and_then(|program| program.accept(self));

        match &result {
            Ok(value) => {
                if self.repl {
                    writeln!(self.output, "{value}")?;
                }
            }
            Err(error) => writeln!(self.output, "{error}")?,
        };

        result
    }

    fn get_function(&self, name: &Token) -> Result<Rc<dyn Callable>> {
        let function = self.environment.get_function(name.lexeme());
        function.ok_or(Error::new(
            name.line_no,
            name.col_no,
            ErrorKind::UndefinedSymbol(name.lexeme().to_owned()),
        ))
    }

    fn evaluate_symbol(&self, name: &Token) -> Result<Value> {
        if let Some(value) = self.environment.get(name.lexeme()) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                name.line_no,
                name.col_no,
                ErrorKind::UndefinedSymbol(name.lexeme().to_owned()),
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
}

impl<'i> Visitor<Result<Value>> for Interpreter<'i> {
    fn visit_define(&mut self, define: &Define) -> Result<Value> {
        todo!()
    }

    fn visit_if(&mut self, if_expr: &If) -> Result<Value> {
        todo!()
    }

    fn visit_while(&mut self, while_expr: &While) -> Result<Value> {
        todo!()
    }

    fn visit_lambda(&mut self, lambda: &Lambda) -> Result<Value> {
        todo!()
    }

    fn visit_call(&mut self, call: &Call) -> Result<Value> {
        todo!()
    }

    fn visit_quoted_datum(&mut self, atom: &Datum) -> Result<Value> {
        todo!()
    }

    fn visit_datum(&mut self, atom: &Datum) -> Result<Value> {
        todo!()
    }
    // fn visit_program(&mut self, program: &Program) -> Result<Value> {
    //     program.expression.accept(self)
    // }

    // fn visit_list(&mut self, list: &List) -> Result<Value> {
    //     let elements = &list.expressions;

    //     let value = if elements.is_empty() {
    //         Value::Nil
    //     } else {
    //         let values = elements
    //             .iter()
    //             .map(|node| node.accept(self))
    //             .collect::<Result<Vec<_>>>()?;

    //         Value::List(values)
    //     };

    //     Ok(value)
    // }

    // fn visit_if_expression(
    //     &mut self,
    //     if_expression: &IfExpression,
    // ) -> Result<Value> {
    //     let condition = if_expression.condition.accept(self)?;

    //     if condition.is_truthy() {
    //         if_expression.then_branch.accept(self)
    //     } else if let Some(else_branch) = &if_expression.else_branch {
    //         else_branch.accept(self)
    //     } else {
    //         Ok(Value::Nil)
    //     }
    // }

    // fn visit_while_expression(
    //     &mut self,
    //     while_expression: &WhileExpression,
    // ) -> Result<Value> {
    //     let mut value = Value::Nil;

    //     loop {
    //         let condition = while_expression.condition.accept(self)?;

    //         if condition.is_truthy() {
    //             value = while_expression.expression.accept(self)?;
    //         } else {
    //             break;
    //         }
    //     }

    //     Ok(value)
    // }

    // fn visit_set_expression(
    //     &mut self,
    //     set_expression: &SetExpression,
    // ) -> Result<Value> {
    //     let SetExpression { name, expression } = set_expression;

    //     let value = expression.accept(self)?;

    //     self.environment.insert(name.lexeme().to_owned(), value);

    //     Ok(Value::Symbol(name.lexeme().to_owned()))
    // }

    // fn visit_function_call(
    //     &mut self,
    //     function_call: &FunctionCall,
    // ) -> Result<Value> {
    //     let function = self.get_function(&function_call.name)?;

    //     function.call(self, &function_call.arguments)
    // }

    // fn visit_function_definition(
    //     &mut self,
    //     function_definition: &FunctionDefinition,
    // ) -> Result<Value> {
    //     let name = function_definition.name.lexeme().to_owned();

    //     let parameters = Parameters::new(
    //         function_definition
    //             .parameters
    //             .iter()
    //             .map(|param| {
    //                 Parameter::new(
    //                     param.lexeme().to_owned(),
    //                     vec![ParameterType::Any],
    //                     false,
    //                 )
    //             })
    //             .collect(),
    //     )?;

    //     let function_value = Function::new(
    //         name,
    //         parameters,
    //         function_definition.expression.clone(),
    //     );

    //     self.environment.insert(
    //         function_definition.name.lexeme().to_owned(),
    //         function_value.into(),
    //     );

    //     Ok(Value::Nil)
    // }

    // fn visit_atom(&mut self, atom: &Atom) -> Result<Value> {
    //     let value = match atom {
    //         Atom::Boolean(token) => Value::Boolean(token.as_bool().unwrap()),
    //         Atom::Number(token) => Value::Number(token.as_number().unwrap()),
    //         Atom::String(token) => Value::String(token.lexeme().to_owned()),
    //         Atom::Symbol(token) => self.evaluate_symbol(token)?,
    //         Atom::Nil(_) => Value::Nil,
    //     };

    //     Ok(value)
    // }
}
