use crate::{
    parser::parse_string, visitor::visit_node_fallible, Node, Result, Visitor,
};

mod environment;

#[derive(Debug, Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, source: &str, name: &str) -> Result<Node> {
        let node = parse_string(source, name)?;

        node.accept(self)
    }
}

impl Visitor<Result<Node>> for Interpreter {
    fn visit_node(&mut self, node: &Node) -> Result<Node> {
        visit_node_fallible(self, node)
    }

    fn visit_main(&mut self, nodes: &[Node]) -> Result<Node> {
        todo!()
    }

    fn visit_table(
        &mut self,
        map: &std::collections::HashMap<Node, Node>,
    ) -> Result<Node> {
        todo!()
    }

    fn visit_p_array(&mut self, nodes: &[Node]) -> Result<Node> {
        todo!()
    }

    fn visit_b_array(&mut self, nodes: &[Node]) -> Result<Node> {
        todo!()
    }

    fn visit_struct(
        &mut self,
        map: &std::collections::HashMap<Node, Node>,
    ) -> Result<Node> {
        todo!()
    }

    fn visit_p_tuple(&mut self, nodes: &[Node]) -> Result<Node> {
        todo!()
    }

    fn visit_b_tuple(&mut self, nodes: &[Node]) -> Result<Node> {
        todo!()
    }

    fn visit_number(&mut self, number: f64) -> Result<Node> {
        todo!()
    }

    fn visit_string(&mut self, string: &str) -> Result<Node> {
        todo!()
    }

    fn visit_buffer(&mut self, buffer: &str) -> Result<Node> {
        todo!()
    }

    fn visit_symbol(&mut self, symbol: &str) -> Result<Node> {
        todo!()
    }

    fn visit_keyword(&mut self, keyword: &str) -> Result<Node> {
        todo!()
    }

    fn visit_true(&mut self) -> Result<Node> {
        todo!()
    }

    fn visit_false(&mut self) -> Result<Node> {
        todo!()
    }

    fn visit_nil(&mut self) -> Result<Node> {
        todo!()
    }
}
