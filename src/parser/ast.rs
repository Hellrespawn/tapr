use crate::interpreter::visitor::Visitor;

pub trait Node: std::fmt::Debug {
    fn accept_visitor(&self, visitor: &dyn Visitor) -> String;
}

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Expression>,
}

impl Node for Program {
    fn accept_visitor(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_program(self)
    }
}

#[derive(Debug)]
pub struct Expression {
    pub symbol: String,
    pub arguments: Vec<Box<dyn Node>>,
}

impl Node for Expression {
    fn accept_visitor(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_expression(self)
    }
}

#[derive(Debug)]
pub struct List {
    pub elements: Vec<Box<dyn Node>>,
}

impl Node for List {
    fn accept_visitor(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_list(self)
    }
}

#[derive(Debug)]
pub enum Atom {
    Number(f64),
    String(String),
    Symbol(String),
}

impl Node for Atom {
    fn accept_visitor(&self, visitor: &dyn Visitor) -> String {
        visitor.visit_atom(self)
    }
}
