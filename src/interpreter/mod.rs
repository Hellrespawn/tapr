mod builtin;
mod environment;
mod value;

pub mod parameters;

pub use value::Value;

use self::environment::Environment;
use self::parameters::{Parameter, Parameters};
use self::value::Lambda;
use crate::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::parser::{ast, Parser};
use crate::token::Token;
use crate::visitor::Visitor;
use crate::Result;
use std::io::Write;

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

    fn get(&self, name: &Token) -> Result<Value> {
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
    fn visit_define(&mut self, define: &ast::Define) -> Result<Value> {
        let ast::Define { name, expression } = define;

        let value = expression.accept(self)?;

        self.environment.insert(name.0.lexeme().to_owned(), value);

        Ok(Value::Symbol(name.0.lexeme().to_owned()))
    }

    fn visit_if(&mut self, if_expr: &ast::If) -> Result<Value> {
        let condition = if_expr.condition.accept(self)?;

        if condition.is_truthy() {
            if_expr.then_branch.accept(self)
        } else if let Some(else_branch) = &if_expr.else_branch {
            else_branch.accept(self)
        } else {
            Ok(Value::Nil)
        }
    }

    fn visit_while(&mut self, while_expr: &ast::While) -> Result<Value> {
        let ast::While {
            condition,
            expression,
        } = while_expr;

        let mut list = Vec::new();

        while condition.accept(self)?.is_truthy() {
            list.push(expression.accept(self)?);
        }

        Ok(Value::List(list))
    }

    fn visit_lambda(&mut self, lambda: &ast::Lambda) -> Result<Value> {
        let parameters = Parameters::new(
            lambda
                .parameters
                .iter()
                .map(|s| Parameter::any(s.0.lexeme()))
                .collect(),
        )?;

        Ok(Value::Lambda(Lambda::new(
            parameters,
            lambda.expression.clone(),
        )))
    }

    fn visit_call(&mut self, call: &ast::Call) -> Result<Value> {
        self.enter_scope();

        let ast::Call { symbol, arguments } = call;

        let value = self.get(&symbol.0)?;

        let result = match value {
            Value::Builtin(builtin) => builtin.call(self, arguments),
            Value::Lambda(lambda) => lambda.call(self, arguments),
            _ => Err(Error::new(
                symbol.0.line_no,
                symbol.0.col_no,
                ErrorKind::NotCallable(value),
            )),
        };

        self.exit_scope();

        result
    }

    fn visit_quoted_datum(&mut self, datum: &ast::Datum) -> Result<Value> {
        let value = match datum {
            ast::Datum::Symbol(symbol) => {
                Value::Symbol(symbol.0.lexeme().to_owned())
            }
            datum => self.visit_datum(datum)?,
        };

        Ok(value)
    }

    fn visit_datum(&mut self, datum: &ast::Datum) -> Result<Value> {
        let value = match datum {
            ast::Datum::List(list) => {
                let list = list
                    .expressions
                    .iter()
                    .map(|e| e.accept(self))
                    .collect::<Result<Vec<_>>>()?;

                Value::List(list)
            }
            ast::Datum::Boolean(boolean) => Value::Boolean(
                boolean
                    .0
                    .lexeme()
                    .parse()
                    .expect("Lexer should have checked boolean validity."),
            ),
            ast::Datum::Number(number) => Value::Number(
                number
                    .0
                    .lexeme()
                    .parse()
                    .expect("Lexer should have checked number validity."),
            ),
            ast::Datum::String(string) => {
                Value::String(string.0.lexeme().to_owned())
            }
            ast::Datum::Symbol(symbol) => self.get(&symbol.0)?,
            ast::Datum::Nil => Value::Nil,
        };

        Ok(value)
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
