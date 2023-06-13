use crate::parser::ast::Node;

pub trait Visitor<T: std::fmt::Debug> {
    fn visit_node(&mut self, node: &Node) -> T;
}
