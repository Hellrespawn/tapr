use crate::parser::ast::{Node, NodeData, Special};
use crate::visitor::Visitor;
use std::process::Command;

pub(crate) struct GraphVisitor {
    counter: usize,
    body: String,
}

impl GraphVisitor {
    pub(crate) fn create_ast_graph(node: &Node, name: &str) {
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

        GraphVisitor::write_graph_to_file(visitor.body, &filename);
    }

    fn write_graph_to_file(dot: String, filename: &str) {
        let result = std::fs::write(filename, dot);

        if result.is_err() {
            eprintln!("Unable to write to dot-file to {filename}.");
        }

        if which::which("dot").is_ok() {
            GraphVisitor::render_graph(filename);
            if std::fs::remove_file(filename).is_err() {
                eprintln!("Unable to remove dot-file after rendering.");
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

    // fn hidden_node(&mut self) -> usize {
    //     self.create_node("", true)
    // }

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

    fn accept_and_connect_many_with_label(
        &mut self,
        parent_node: usize,
        to_visit: &[Node],
        label: &str,
    ) {
        for node in to_visit {
            self.accept_and_connect_with_label(parent_node, node, label);
        }
    }
}

impl Visitor<()> for GraphVisitor {
    fn visit_node(&mut self, node: &Node) {
        match node.data() {
            NodeData::Main(nodes) => self.visit_main(nodes),
            NodeData::Special(special) => match &**special {
                Special::If {
                    condition,
                    then,
                    else_branch,
                } => self.visit_if(condition, then, else_branch.as_ref()),
                Special::Fn { parameters, body } => {
                    self.visit_fn(parameters, body);
                }
                Special::Set { name, value } => {
                    self.visit_set(name, value, node.location());
                }
                Special::Var { name, value } => {
                    self.visit_var(name, value, node.location());
                }
            },
            NodeData::List { literal, nodes } => {
                self.visit_list(*literal, nodes);
            }
            NodeData::Symbol { module, value } => {
                let symbol = if let Some(module) = module {
                    format!("{module}/{value}")
                } else {
                    value.clone()
                };

                self.new_node(&format!("symbol\n{symbol}"));
            }
            NodeData::Keyword(keyword) => {
                self.new_node(&format!("keyword:\n{keyword}"));
            }
            NodeData::Number(number) => {
                self.new_node(&format!("number:\n{number}"));
            }
            NodeData::String(string) => {
                self.new_node(&format!("string:\n\\\"{string}\\\""));
            }
            NodeData::True => {
                self.new_node("true");
            }
            NodeData::False => {
                self.new_node("false");
            }
            NodeData::Nil => {
                self.new_node("nil");
            }
        }
    }

    fn visit_main(&mut self, nodes: &[Node]) {
        let parent_node = self.new_node("main");

        self.accept_and_connect_many(parent_node, nodes);
    }

    fn visit_if(
        &mut self,
        condition: &Node,
        then: &Node,
        else_branch: Option<&Node>,
    ) {
        let parent_node = self.new_node("if");

        self.accept_and_connect_with_label(parent_node, condition, "condition");

        self.accept_and_connect_with_label(parent_node, then, "then");

        if let Some(else_branch) = else_branch {
            self.accept_and_connect_with_label(
                parent_node,
                else_branch,
                "else",
            );
        }
    }

    fn visit_fn(&mut self, parameters: &[String], body: &[Node]) {
        let parent_node =
            self.new_node(&format!("fn\n[{}]", parameters.join(", ")));

        self.accept_and_connect_many_with_label(parent_node, body, "body");
    }

    fn visit_set(
        &mut self,
        name: &str,
        value: &Node,
        _location: crate::location::Location,
    ) {
        let parent_node = self.new_node(&format!("set\n'{name}'"));

        self.accept_and_connect(parent_node, value);
    }

    fn visit_var(
        &mut self,
        name: &str,
        value: &Node,
        _location: crate::location::Location,
    ) {
        let parent_node = self.new_node(&format!("var\n'{name}'"));

        self.accept_and_connect(parent_node, value);
    }

    fn visit_list(&mut self, literal: bool, nodes: &[Node]) {
        let parent_node = self.new_node(if literal { "list" } else { "form" });

        self.accept_and_connect_many(parent_node, nodes);
    }
}
