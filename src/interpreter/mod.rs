use crate::builtin::get_builtin_function;
use crate::parser::ast::{self, Atom, Program};

use self::visitor::Visitor;

pub mod visitor;

// const KEYWORDS: [&str; 1] = ["function"];

pub struct Interpreter {}

// impl Interpreter {
//     pub fn is_keyword(atom: Atom) -> bool {
//         match atom {
//             Atom::Symbol(string) => KEYWORDS.contains(&&*string),
//             _ => false,
//         }
//     }
// }

impl Interpreter {
    pub fn interpret(&self, program: &Program) -> String {
        self.visit_program(program)
    }

    fn error(message: &str) -> ! {
        panic!("{}", message)
    }
}

impl Visitor for Interpreter {
    fn visit_program(&self, program: &Program) -> String {
        program
            .expressions
            .iter()
            .map(|expression| self.visit_expression(expression))
            .collect::<String>()
    }

    fn visit_expression(&self, expression: &ast::Expression) -> String {
        let arguments = expression
            .arguments
            .iter()
            .map(|n| n.accept_visitor(self))
            .collect::<Vec<_>>();

        if let Some(function) = get_builtin_function(&expression.symbol) {
            function(&arguments)
        } else {
            Self::error("Unable to find function")
        }
    }

    fn visit_list(&self, list: &ast::List) -> String {
        if list.elements.is_empty() {
            return String::new();
        }

        list.elements
            .iter()
            .map(|n| n.accept_visitor(self))
            .collect::<String>()
    }

    fn visit_atom(&self, atom: &Atom) -> String {
        match atom {
            Atom::Number(number) => number.to_string(),
            Atom::String(string) => string.clone(),
            Atom::Symbol(symbol) => symbol.clone(),
        }
    }
}
