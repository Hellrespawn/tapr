use crate::parser::ast::{Atom, List, Program};

pub trait Visitor<T: std::fmt::Debug> {
    fn visit_program(&self, program: &Program) -> T;
    fn visit_list(&self, list: &List) -> T;
    fn visit_atom(&self, atom: &Atom) -> T;
}
