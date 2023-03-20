use crate::parser::ast::*;
use crate::visitor::Visitor;
use std::process::Command;

pub(crate) struct GraphVisitor {
    counter: usize,
    body: String,
}

impl GraphVisitor {
    pub(crate) fn create_ast_graph(program: &Expression, filename: &str) {
        let body = "digraph astgraph {\n  \
            edge [arrowsize=.5];\n  \
            rankdir=\"LR\";\n  \
            newrank=true;\n  \
            nodesep=0.75;\n  \
            ranksep=0.75;\n  "
            .to_owned();

        let mut visitor = Self { counter: 0, body };

        program.accept(&mut visitor);

        visitor.body.push('}');

        GraphVisitor::write_graph_to_file(visitor.body, filename);
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

    fn accept_and_connect(
        &mut self,
        parent_node: usize,
        to_visit: &Expression,
    ) {
        let new_node = self.counter;
        to_visit.accept(self);

        self.connect_nodes(parent_node, new_node);
    }

    fn accept_and_connect_with_label(
        &mut self,
        parent_node: usize,
        to_visit: &Expression,
        label: &str,
    ) {
        let new_node = self.counter;
        to_visit.accept(self);

        self.connect_nodes_with_label(parent_node, new_node, label);
    }

    fn accept_and_connect_many(
        &mut self,
        parent_node: usize,
        to_visit: &[Expression],
    ) {
        for node in to_visit {
            self.accept_and_connect(parent_node, node);
        }
    }

    fn accept_and_connect_many_with_label(
        &mut self,
        parent_node: usize,
        to_visit: &[Expression],
        label: &str,
    ) {
        for node in to_visit {
            self.accept_and_connect_with_label(parent_node, node, label);
        }
    }
}

impl Visitor<()> for GraphVisitor {
    fn visit_define(&mut self, define: &Define) {
        let parent_node = self.new_node("define");

        self.accept_and_connect_with_label(
            parent_node,
            &Expression::Datum(Datum::Symbol(define.name.clone())),
            "symbol",
        );

        self.accept_and_connect_with_label(
            parent_node,
            &define.expression,
            "expression",
        );
    }

    fn visit_if(&mut self, if_expr: &If) {
        let parent_node = self.new_node("if");

        self.accept_and_connect_with_label(
            parent_node,
            &if_expr.condition,
            "condition",
        );

        self.accept_and_connect_with_label(
            parent_node,
            &if_expr.then_branch,
            "then",
        );

        if let Some(else_branch) = &if_expr.else_branch {
            self.accept_and_connect_with_label(
                parent_node,
                else_branch,
                "else",
            );
        }
    }

    fn visit_while(&mut self, while_expr: &While) {
        let parent_node = self.new_node("while");

        self.accept_and_connect_with_label(
            parent_node,
            &while_expr.condition,
            "condition",
        );

        self.accept_and_connect_with_label(
            parent_node,
            &while_expr.expression,
            "expression",
        );
    }

    fn visit_lambda(&mut self, lambda: &Lambda) {
        let parent_node = self.new_node("lambda");

        let symbols = lambda
            .parameters
            .iter()
            .map(|s| Expression::Datum(Datum::Symbol(s.clone())))
            .collect::<Vec<_>>();

        self.accept_and_connect_many_with_label(
            parent_node,
            &symbols,
            "parameter",
        );

        self.accept_and_connect_with_label(
            parent_node,
            &lambda.expression,
            "expression",
        );
    }

    fn visit_call(&mut self, call: &Call) {
        let parent_node = self.new_node("call");

        self.accept_and_connect_with_label(
            parent_node,
            &Expression::Datum(Datum::Symbol(call.symbol.clone())),
            "symbol",
        );

        self.accept_and_connect_many_with_label(
            parent_node,
            &call.arguments,
            "expression",
        );
    }

    fn visit_quoted_datum(&mut self, datum: &Datum) {
        let parent_node = self.new_node("quote");

        let counter = self.counter;

        self.visit_datum(datum);

        self.connect_nodes(parent_node, counter);
    }

    fn visit_datum(&mut self, datum: &Datum) {
        match datum {
            Datum::List(list) => {
                let parent_node = self.new_node("list");

                self.accept_and_connect_many(parent_node, &list.expressions);
            }
            Datum::Boolean(bool) => {
                self.new_node(&format!("Boolean:\n{}", bool.0.lexeme()));
            }
            Datum::Number(number) => {
                self.new_node(&format!("Number:\n{}", number.0.lexeme()));
            }
            Datum::String(string) => {
                self.new_node(&format!("String:\n{}", string.0.lexeme()));
            }
            Datum::Symbol(symbol) => {
                self.new_node(&format!("Symbol:\n{}", symbol.0.lexeme()));
            }
            Datum::Nil => {
                self.new_node("nil");
            }
        }
    }
}
