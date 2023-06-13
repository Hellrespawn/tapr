use crate::location::Location;
use crate::parser::ast::Node;

pub trait Visitor<T: std::fmt::Debug> {
    fn visit_node(&mut self, node: &Node) -> T;
    fn visit_main(&mut self, nodes: &[Node]) -> T;
    fn visit_if(
        &mut self,
        condition: &Node,
        then: &Node,
        else_branch: Option<&Node>,
    ) -> T;
    fn visit_fn(&mut self, parameters: &[String], body: &[Node]) -> T;
    fn visit_set(&mut self, name: &str, value: &Node, location: Location) -> T;
    fn visit_var(&mut self, name: &str, value: &Node, location: Location) -> T;
    fn visit_list(&mut self, literal: bool, nodes: &[Node]) -> T;
}
