use crate::parser::ast::{Atom, Expression, List, Program};

pub trait Visitor {
    fn visit_program(&self, program: &Program) -> String;
    fn visit_expression(&self, expression: &Expression) -> String;
    fn visit_list(&self, list: &List) -> String;
    fn visit_atom(&self, atom: &Atom) -> String;
}
