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
    repl: bool,
}

impl<'i> Default for Interpreter<'i> {
    fn default() -> Self {
        Self {
            output: Box::new(std::io::stdout()),
            environment: Environment::root(),
            repl: false,
        }
    }
}

impl<'i> Interpreter<'i> {
    pub fn new(
        output: Box<dyn Write + 'i>,
        environment: Environment,
        repl: bool,
    ) -> Self {
        Self {
            output,
            environment,
            repl,
        }
    }

    pub fn repl() -> Self {
        Self::new(Box::new(std::io::stdout()), Environment::root(), true)
    }

    pub fn interpret(&mut self, source: &str) -> Result<Value> {
        let lexer = Lexer::new(source);

        let mut parser = Parser::new(lexer);

        let result = parser
            .parse()
            .and_then(|expression| expression.accept(self));

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
            Value::Builtin(builtin) => {
                builtin.call(self, arguments).map_err(|mut e| {
                    e.line_no = Some(symbol.0.line_no);
                    e.col_no = Some(symbol.0.col_no);
                    e
                })
            }
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
        // Quoted list is handled on the parser level.
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
}
