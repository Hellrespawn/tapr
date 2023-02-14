mod function;
mod value;
mod visitor;

use std::collections::HashMap;

pub use value::Value;
pub use visitor::Visitor;

use crate::parser::ast::*;
use crate::{Error, Result};
use function::{Function, BUILTIN_FUNCTIONS};

pub struct Interpreter {
    environment: HashMap<String, Value>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: &Node) -> Result<Value> {
        program.accept(self)
    }

    fn get_function(name: &str) -> Option<&dyn Function> {
        let function = BUILTIN_FUNCTIONS.get(name);

        function.map(|f| &**f)
    }

    fn get_variable(&self, name: &str) -> Option<Value> {
        self.environment.get(name).cloned()
    }

    fn evaluate_function(name: &str, arguments: &[Value]) -> Result<Value> {
        let function = Interpreter::get_function(name)
            .expect("Existence of function is checked by `evaluate_symbol`");

        function.call(arguments)
    }

    fn evaluate_symbol(&self, name: &str) -> Result<Value> {
        if let Some(value) = self.get_variable(name) {
            Ok(value)
        } else if Interpreter::get_function(name).is_some() {
            // Evaluated afterward
            Ok(Value::Symbol(name.to_owned()))
        } else {
            Err(Error::UndefinedSymbol(name.to_owned()))
        }
    }
}

impl Visitor<Result<Value>> for Interpreter {
    fn visit_program(&mut self, program: &Program) -> Result<Value> {
        let values = program
            .expressions
            .iter()
            .map(|node| node.accept(self))
            .collect::<Result<Vec<_>>>()?;

        Ok(Value::List(values))
    }

    fn visit_if_expression(
        &mut self,
        if_expression: &IfExpression,
    ) -> Result<Value> {
        let condition = if_expression.condition.accept(self)?;

        if condition.is_truthy() {
            if_expression.then_branch.accept(self)
        } else if let Some(else_branch) = &if_expression.else_branch {
            else_branch.accept(self)
        } else {
            Ok(Value::Nil)
        }
    }

    fn visit_set_expression(
        &mut self,
        set_expression: &SetExpression,
    ) -> Result<Value> {
        let SetExpression { symbol, expression } = set_expression;

        let value = expression.accept(self)?;

        self.environment.insert(symbol.clone(), value);

        Ok(Value::Nil)
    }

    fn visit_list(&mut self, list: &List) -> Result<Value> {
        let elements = &list.elements;

        let value = if elements.is_empty() {
            Value::Nil
        } else {
            let values = elements
                .iter()
                .map(|node| node.accept(self))
                .collect::<Result<Vec<_>>>()?;

            if let Some(Some(symbol)) = values.first().map(Value::get_symbol) {
                Interpreter::evaluate_function(&symbol, &values[1..])?
            } else {
                Value::List(values)
            }
        };

        Ok(value)
    }

    fn visit_atom(&mut self, atom: &Atom) -> Result<Value> {
        let value = match atom {
            Atom::Boolean(bool) => Value::Boolean(*bool),
            Atom::Number(number) => Value::Number(*number),
            Atom::String(string) => Value::String(string.clone()),
            Atom::Symbol(symbol) => self.evaluate_symbol(symbol)?,
            Atom::Nil => Value::Nil,
        };

        Ok(value)
    }
}
