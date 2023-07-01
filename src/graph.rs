use crate::visitor::Visitor;
use crate::{Node, NodeData};
use std::process::Command;

pub(crate) struct GraphVisitor {
    counter: usize,
    body: String,
}

impl GraphVisitor {
    pub(crate) fn create_ast_graph(node: &Node, name: &str, retain_dot: bool) {
        let body = "digraph astgraph {\n  \
            edge [arrowsize=.5];\n  \
            rankdir=\"LR\";\n  \
            newrank=true;\n  \
            nodesep=0.75;\n  \
            ranksep=0.75;\n  "
            .to_owned();

        let mut visitor = Self { counter: 0, body };

        node.accept(&mut visitor);

        visitor.body.push('}');

        let filename = format!("{name}.ast.dot");

        GraphVisitor::write_graph_to_file(visitor.body, &filename, retain_dot);
    }

    fn write_graph_to_file(dot: String, filename: &str, retain_dot: bool) {
        let result = std::fs::write(filename, dot);

        if result.is_err() {
            eprintln!("Unable to write to dot-file to {filename}.");
        }

        if which::which("dot").is_ok() {
            GraphVisitor::render_graph(filename);
            if !retain_dot {
                let result = std::fs::remove_file(filename);

                if result.is_err() {
                    eprintln!("Unable to remove dot-file after rendering.");
                }
            }
        }
    }

    fn render_graph(filename_in: &str) {
        let filename_out = filename_in.replace(".dot", ".png");

        let mut command = Command::new("dot");

        command.arg("-Tpng");
        command.arg(filename_in);
        command.arg("-o");
        command.arg(filename_out);

        if let Ok(status) = command.status() {
            if !status.success() {
                eprintln!("Something went wrong with GraphViz dot.");
            }
        } else {
            eprintln!("Unable to run GraphViz dot.");
        }
    }

    fn increment(&mut self) -> usize {
        self.counter += 1;
        self.counter - 1
    }

    fn create_node(&mut self, label: &str, hidden: bool) -> usize {
        let label_str = {
            if hidden {
                "[shape=point]".to_owned()
            } else {
                format!(r#"[label="{}"]"#, label.replace('\n', "\\n"))
            }
        };

        self.body
            .push_str(&format!("  node{} {label_str}\n", self.counter));

        self.increment()
    }

    fn new_node(&mut self, label: &str) -> usize {
        self.create_node(label, false)
    }

    fn hidden_node(&mut self) -> usize {
        self.create_node("", true)
    }

    fn node_connector(
        &mut self,
        node1: usize,
        node2: usize,
        label: Option<&str>,
        directed: bool,
    ) {
        self.body.push_str(&format!("  node{node1} -> node{node2}"));

        let mut args: Vec<String> = Vec::new();

        if let Some(label) = label {
            args.push(format!(r#"label="{}""#, label.replace('\n', "\\n")));
        }

        if !directed {
            args.push("dir=none".to_owned());
        }

        if !args.is_empty() {
            self.body.push_str(&format!(" [{}]", args.join(", ")));
        }

        self.body.push('\n');
    }

    fn connect_nodes(&mut self, node1: usize, node2: usize) {
        self.node_connector(node1, node2, None, true);
    }

    fn connect_nodes_with_label(
        &mut self,
        node1: usize,
        node2: usize,
        label: &str,
    ) {
        self.node_connector(node1, node2, Some(label), true);
    }

    fn accept_and_connect(&mut self, parent_node: usize, to_visit: &Node) {
        let new_node = self.counter;
        to_visit.accept(self);

        self.connect_nodes(parent_node, new_node);
    }

    fn accept_and_connect_with_label(
        &mut self,
        parent_node: usize,
        to_visit: &Node,
        label: &str,
    ) {
        let new_node = self.counter;
        to_visit.accept(self);

        self.connect_nodes_with_label(parent_node, new_node, label);
    }

    fn accept_and_connect_many(
        &mut self,
        parent_node: usize,
        to_visit: &[Node],
    ) {
        for node in to_visit {
            self.accept_and_connect(parent_node, node);
        }
    }

    // fn accept_and_connect_many_with_label(
    //     &mut self,
    //     parent_node: usize,
    //     to_visit: &[Node],
    //     label: &str,
    // ) {
    //     for node in to_visit {
    //         self.accept_and_connect_with_label(parent_node, node, label);
    //     }
    // }

    fn visit_main(&mut self, nodes: &[Node]) {
        let parent_node = self.new_node("main");

        self.accept_and_connect_many(parent_node, nodes);
    }

    fn visit_table(&mut self, map: &std::collections::HashMap<Node, Node>) {
        let parent_node = self.new_node("table");

        for (key, value) in map {
            let pair_node = self.hidden_node();

            self.connect_nodes(pair_node, parent_node);

            self.accept_and_connect_with_label(pair_node, key, "key");

            self.accept_and_connect_with_label(pair_node, value, "value");
        }
    }

    fn visit_p_array(&mut self, nodes: &[Node]) {
        let parent_node = self.new_node("p_array");

        self.accept_and_connect_many(parent_node, nodes);
    }

    fn visit_b_array(&mut self, nodes: &[Node]) {
        let parent_node = self.new_node("b_array");

        self.accept_and_connect_many(parent_node, nodes);
    }

    fn visit_struct(&mut self, map: &std::collections::HashMap<Node, Node>) {
        let parent_node = self.new_node("struct");

        for (key, value) in map {
            let pair_node = self.hidden_node();

            self.connect_nodes(parent_node, pair_node);

            self.accept_and_connect_with_label(pair_node, key, "key");

            self.accept_and_connect_with_label(pair_node, value, "value");
        }
    }

    fn visit_p_tuple(&mut self, nodes: &[Node]) {
        let parent_node = self.new_node("p_tuple");

        self.accept_and_connect_many(parent_node, nodes);
    }

    fn visit_b_tuple(&mut self, nodes: &[Node]) {
        let parent_node = self.new_node("b_tuple");

        self.accept_and_connect_many(parent_node, nodes);
    }

    fn visit_number(&mut self, number: f64) {
        self.new_node(&format!("{number}"));
    }

    fn visit_string(&mut self, string: &str) {
        self.new_node(&format!("\\\"{string}\\\""));
    }

    fn visit_buffer(&mut self, buffer: &str) {
        self.new_node(&format!("@\\\"{buffer}\\\""));
    }

    fn visit_symbol(&mut self, symbol: &str) {
        self.new_node(symbol);
    }

    fn visit_keyword(&mut self, keyword: &str) {
        self.new_node(&format!(":{keyword}"));
    }

    fn visit_true(&mut self) {
        self.new_node("true");
    }

    fn visit_false(&mut self) {
        self.new_node("false");
    }

    fn visit_nil(&mut self) {
        self.new_node("nil");
    }
}

impl Visitor<()> for GraphVisitor {
    fn visit_node(&mut self, node: &Node) {
        match node.data() {
            NodeData::Main(nodes) => self.visit_main(nodes),
            NodeData::Table(map) => self.visit_table(map),
            NodeData::PArray(nodes) => self.visit_p_array(nodes),
            NodeData::BArray(nodes) => self.visit_b_array(nodes),
            NodeData::Struct(map) => self.visit_struct(map),
            NodeData::PTuple(nodes) => self.visit_p_tuple(nodes),
            NodeData::BTuple(nodes) => self.visit_b_tuple(nodes),
            NodeData::Number(number) => self.visit_number(*number),
            NodeData::String(string) => self.visit_string(string),
            NodeData::Buffer(buffer) => self.visit_buffer(buffer),
            NodeData::Symbol(symbol) => self.visit_symbol(symbol),
            NodeData::Keyword(keyword) => self.visit_keyword(keyword),
            NodeData::True => self.visit_true(),
            NodeData::False => self.visit_false(),
            NodeData::Nil => self.visit_nil(),
        }
    }
}
