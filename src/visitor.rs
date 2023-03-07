use crate::parser::ast::*;

pub trait Visitor<T: std::fmt::Debug> {
    fn visit_define(&mut self, define: &Define) -> T;
    fn visit_if(&mut self, if_expr: &If) -> T;
    fn visit_while(&mut self, while_expr: &While) -> T;
    fn visit_lambda(&mut self, lambda: &Lambda) -> T;
    fn visit_call(&mut self, call: &Call) -> T;
    fn visit_quoted_datum(&mut self, datum: &Datum) -> T;
    fn visit_datum(&mut self, datum: &Datum) -> T;
}
