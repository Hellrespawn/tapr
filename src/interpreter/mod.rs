mod function;
mod value;
mod visitor;

pub use value::Value;
pub use visitor::Visitor;

use crate::parser::ast::{Atom, List, Node, Program};
use crate::{Error, Result};
use function::{Function, BUILTIN_FUNCTIONS};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, program: &Node) -> Result<Value> {
        program.accept(self)
    }

    fn get_function(name: &str) -> Option<&dyn Function> {
        let function = BUILTIN_FUNCTIONS.get(name);

        function.map(|f| &**f)
    }

    fn get_variable(&self, _name: &str) -> Option<Value> {
        None
    }

    fn evaluate_function(name: &str, arguments: &[Value]) -> Result<Value> {
        let function = Interpreter::get_function(name)
            .expect("Existence of function is checked by `evaluate_symbol`");

        function.call(arguments)
    }

    fn evaluate_symbol(&self, name: &str) -> Result<Value> {
        if let Some(value) = self.get_variable(name) {
            Ok(value)
        } else if let Some(_function) = Interpreter::get_function(name) {
            // Evaluated afterward
            Ok(Value::Symbol(name.to_owned()))
        } else {
            Err(Error::UndefinedSymbol(name.to_owned()))
        }
    }
}

impl Visitor<Result<Value>> for Interpreter {
    fn visit_program(&self, program: &Program) -> Result<Value> {
        let values = program
            .lists
            .iter()
            .map(|list| self.visit_list(list))
            .collect::<Result<Vec<_>>>()?;

        Ok(Value::List(values))
    }

    fn visit_list(&self, list: &List) -> Result<Value> {
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

    fn visit_atom(&self, atom: &Atom) -> Result<Value> {
        let value = match atom {
            Atom::Number(number) => Value::Number(*number),
            Atom::String(string) => Value::String(string.clone()),
            Atom::Symbol(symbol) => self.evaluate_symbol(symbol)?,
            Atom::Nil => Value::Nil,
        };

        Ok(value)
    }
}
