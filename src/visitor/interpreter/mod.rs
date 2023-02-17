mod function;
mod value;

use std::collections::HashMap;

pub use value::Value;

use crate::parser::ast::*;
use crate::token::Token;
use crate::visitor::Visitor;
use crate::{Error, Result};
use function::{Function, BUILTIN_FUNCTIONS};

pub struct Interpreter {
    pub environment: HashMap<String, Value>,
    pub parser_no: usize,
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
            parser_no: 1,
        }
    }

    pub fn interpret(&mut self, program: &Node) -> Result<Value> {
        program.accept(self)
    }

    fn get_function(name: &Token) -> Result<&dyn Function> {
        let function = BUILTIN_FUNCTIONS.get(name.lexeme());

        function.map(|f| &**f).ok_or(Error::UndefinedSymbol {
            symbol: name.lexeme().to_owned(),
            line_no: name.line_no,
            col_no: name.col_no,
        })
    }

    fn evaluate_symbol(&self, name: &Token) -> Result<Value> {
        if let Some(value) = self.environment.get(name.lexeme()) {
            Ok(value.clone())
        } else {
            Err(Error::UndefinedSymbol {
                symbol: name.lexeme().to_owned(),
                line_no: name.line_no,
                col_no: name.col_no,
            })
        }
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

    fn visit_var_expression(
        &mut self,
        var_expression: &VarExpression,
    ) -> Result<Value> {
        let VarExpression { name, value, scope } = var_expression;

        let value = value.accept(self)?;

        self.environment.insert(name.lexeme().to_owned(), value);

        scope.accept(self)
    }

    fn visit_function_call(
        &mut self,
        function_call: &FunctionCall,
    ) -> Result<Value> {
        let function = Interpreter::get_function(&function_call.name)?;

        function.call(self, &function_call.arguments)
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
