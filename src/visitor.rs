use crate::error::Error;
use crate::location::Location;
use crate::parser::ast::Node;
use crate::NodeData;
use crate::Result;
use std::collections::HashMap;

pub trait Visitor<T: std::fmt::Debug> {
    fn visit_node(&mut self, node: &Node) -> T;
    fn visit_main(&mut self, nodes: &[Node]) -> T;
    fn visit_table(&mut self, map: &HashMap<Node, Node>) -> T;
    fn visit_p_array(&mut self, nodes: &[Node]) -> T;
    fn visit_b_array(&mut self, nodes: &[Node]) -> T;
    fn visit_struct(&mut self, map: &HashMap<Node, Node>) -> T;
    fn visit_p_tuple(&mut self, nodes: &[Node]) -> T;
    fn visit_b_tuple(&mut self, nodes: &[Node]) -> T;
    fn visit_number(&mut self, number: f64) -> T;
    fn visit_string(&mut self, string: &str) -> T;
    fn visit_buffer(&mut self, buffer: &str) -> T;
    fn visit_symbol(&mut self, symbol: &str) -> T;
    fn visit_keyword(&mut self, keyword: &str) -> T;
    fn visit_true(&mut self) -> T;
    fn visit_false(&mut self) -> T;
    fn visit_nil(&mut self) -> T;
}

pub fn visit_node_infallible<T: std::fmt::Debug>(
    visitor: &mut dyn Visitor<T>,
    node: &Node,
) -> T {
    match node.data() {
        NodeData::Main(nodes) => visitor.visit_main(nodes),
        NodeData::Table(map) => visitor.visit_table(map),
        NodeData::PArray(nodes) => visitor.visit_p_array(nodes),
        NodeData::BArray(nodes) => visitor.visit_b_array(nodes),
        NodeData::Struct(map) => visitor.visit_struct(map),
        NodeData::PTuple(nodes) => visitor.visit_p_tuple(nodes),
        NodeData::BTuple(nodes) => visitor.visit_b_tuple(nodes),
        NodeData::Number(number) => visitor.visit_number(*number),
        NodeData::String(string) => visitor.visit_string(string),
        NodeData::Buffer(buffer) => visitor.visit_buffer(buffer),
        NodeData::Symbol(symbol) => visitor.visit_symbol(symbol),
        NodeData::Keyword(keyword) => visitor.visit_keyword(keyword),
        NodeData::True => visitor.visit_true(),
        NodeData::False => visitor.visit_false(),
        NodeData::Nil => visitor.visit_nil(),
    }
}

pub fn visit_node_fallible<T: std::fmt::Debug>(
    visitor: &mut dyn Visitor<Result<T>>,
    node: &Node,
) -> Result<T> {
    let result = match node.data() {
        NodeData::Main(nodes) => visitor.visit_main(nodes),
        NodeData::Table(map) => visitor.visit_table(map),
        NodeData::PArray(nodes) => visitor.visit_p_array(nodes),
        NodeData::BArray(nodes) => visitor.visit_b_array(nodes),
        NodeData::Struct(map) => visitor.visit_struct(map),
        NodeData::PTuple(nodes) => visitor.visit_p_tuple(nodes),
        NodeData::BTuple(nodes) => visitor.visit_b_tuple(nodes),
        NodeData::Number(number) => visitor.visit_number(*number),
        NodeData::String(string) => visitor.visit_string(string),
        NodeData::Buffer(buffer) => visitor.visit_buffer(buffer),
        NodeData::Symbol(symbol) => visitor.visit_symbol(symbol),
        NodeData::Keyword(keyword) => visitor.visit_keyword(keyword),
        NodeData::True => visitor.visit_true(),
        NodeData::False => visitor.visit_false(),
        NodeData::Nil => visitor.visit_nil(),
    };

    result.map_err(|e| add_location_to_error(e, node.location()))
}

fn add_location_to_error(mut error: Error, location: Location) -> Error {
    error.location = error.location.or(Some(location));
    error
}
