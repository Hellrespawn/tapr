use crate::parser::ast::*;

pub mod graph;
pub mod interpreter;

pub trait Visitor<T: std::fmt::Debug> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_if_expression(&mut self, if_expression: &IfExpression) -> T;
    fn visit_var_expression(&mut self, var_expression: &VarExpression) -> T;
    fn visit_function_call(&mut self, function_call: &FunctionCall) -> T;
    fn visit_list(&mut self, list: &List) -> T;
    fn visit_atom(&mut self, atom: &Atom) -> T;
}
