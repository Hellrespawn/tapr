mod builtin;
mod environment;
mod function;
mod value;

pub use value::Value;

use self::environment::Environment;
use self::value::FunctionValue;
use crate::error::{Error, ErrorKind};
use crate::parser::ast::*;
use crate::token::Token;
use crate::visitor::Visitor;
use crate::Result;
use function::Function;
use std::rc::Rc;

pub struct Interpreter {
    environment: Environment,
    parser_no: usize,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::root(),
            parser_no: 1,
        }
    }

    pub fn interpret(&mut self, program: &Node) -> Result<Value> {
        program.accept(self)
    }

    fn get_function_from_environment(
        &self,
        name: &str,
    ) -> Option<Rc<dyn Function>> {
        let value = self.environment.get(name);

        if let Some(Value::Function(function_value)) = value {
            Some(function_value.clone())
        } else {
            None
        }
    }

    fn get_function(&self, name: &Token) -> Result<Rc<dyn Function>> {
        let function = self.get_function_from_environment(name.lexeme());
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

impl Visitor<Result<Value>> for Interpreter {
    fn visit_program(&mut self, program: &Program) -> Result<Value> {
        program.expression.accept(self)
    }

    fn visit_list(&mut self, list: &List) -> Result<Value> {
        let elements = &list.expressions;

        let value = if elements.is_empty() {
            Value::Nil
        } else {
            let values = elements
                .iter()
                .map(|node| node.accept(self))
                .collect::<Result<Vec<_>>>()?;

            Value::List(values)
        };

        Ok(value)
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

    fn visit_while_expression(
        &mut self,
        while_expression: &WhileExpression,
    ) -> Result<Value> {
        let mut value = Value::Nil;

        loop {
            let condition = while_expression.condition.accept(self)?;

            if condition.is_truthy() {
                value = while_expression.then_branch.accept(self)?;
            } else {
                break;
            }
        }

        Ok(value)
    }

    fn visit_set_expression(
        &mut self,
        set_expression: &SetExpression,
    ) -> Result<Value> {
        let SetExpression { variables, scope } = set_expression;

        self.enter_scope();

        for Variable { name, node } in variables {
            let value = node.accept(self)?;

            self.environment.insert(name.lexeme().to_owned(), value);
        }

        let return_value = scope.accept(self)?;

        self.exit_scope();

        Ok(return_value)
    }

    fn visit_function_call(
        &mut self,
        function_call: &FunctionCall,
    ) -> Result<Value> {
        let function = self.get_function(&function_call.name)?;

        function.call(self, &function_call.arguments)
    }

    fn visit_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> Result<Value> {
        let function_value = FunctionValue::new(
            function_definition.name.lexeme().to_owned(),
            function_definition
                .parameters
                .iter()
                .map(|param| param.lexeme().to_owned())
                .collect(),
            function_definition.expression.clone(),
        );

        self.environment.insert(
            function_definition.name.lexeme().to_owned(),
            function_value.into(),
        );

        Ok(Value::Nil)
    }

    fn visit_atom(&mut self, atom: &Atom) -> Result<Value> {
        let value = match atom {
            Atom::Boolean(token) => Value::Boolean(token.as_bool().unwrap()),
            Atom::Number(token) => Value::Number(token.as_number().unwrap()),
            Atom::String(token) => Value::String(token.lexeme().to_owned()),
            Atom::Symbol(token) => self.evaluate_symbol(token)?,
            Atom::Nil(_) => Value::Nil,
        };

        Ok(value)
    }
}
