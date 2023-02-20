use std::process::Command;

use crate::parser::ast::*;
use crate::visitor::Visitor;

pub(crate) struct DotVisitor {
    counter: usize,
    body: String,
}

impl DotVisitor {
    pub(crate) fn create_ast_dot(program: &Node, filename: &str) {
        let body = "digraph astgraph {\n  \
            edge [arrowsize=.5];\n  \
            rankdir=\"TB\";\n  \
            newrank=true;\n  \
            nodesep=0.75;\n  \
            ranksep=0.75;\n  "
            .to_owned();

        let mut dot_visitor = Self { counter: 0, body };

        program.accept(&mut dot_visitor);

        dot_visitor.body.push('}');

        DotVisitor::write_graph_to_file(dot_visitor.body, filename);
    }

    fn write_graph_to_file(dot: String, filename: &str) {
        let result = std::fs::write(filename, dot);

        if result.is_err() {
            eprintln!("Unable to write to dot-file to {filename}.");
        }

        if which::which("dot").is_ok() {
            DotVisitor::render_graph(filename);
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
}

impl Visitor<()> for DotVisitor {
    fn visit_program(&mut self, program: &Program) {
        let program_node = self.new_node("Program");

        let new_node = self.counter;

        program.expression.accept(self);

        self.connect_nodes(program_node, new_node);
    }

    fn visit_if_expression(&mut self, if_expression: &IfExpression) {
        let if_node = self.new_node("If");

        let condition_node = self.counter;
        if_expression.condition.accept(self);

        self.connect_nodes_with_label(if_node, condition_node, "condition");

        let then_branch_node = self.counter;
        if_expression.then_branch.accept(self);

        self.connect_nodes_with_label(if_node, then_branch_node, "then");

        if let Some(else_branch) = &if_expression.else_branch {
            let else_branch_node = self.counter;
            else_branch.accept(self);

            self.connect_nodes_with_label(if_node, else_branch_node, "else");
        }
    }

    fn visit_while_expression(&mut self, while_expression: &WhileExpression) {
        let if_node = self.new_node("While");

        let condition_node = self.counter;
        while_expression.condition.accept(self);

        self.connect_nodes_with_label(if_node, condition_node, "condition");

        let then_branch_node = self.counter;
        while_expression.then_branch.accept(self);

        self.connect_nodes_with_label(if_node, then_branch_node, "then");
    }

    fn visit_set_expression(&mut self, set_expression: &SetExpression) {
        let set_node =
            self.new_node(&format!("Set '{}'", set_expression.name.lexeme()));

        let value_node = self.counter;
        set_expression.value.accept(self);

        self.connect_nodes_with_label(set_node, value_node, "value");

        let scope_node = self.counter;
        set_expression.scope.accept(self);

        self.connect_nodes_with_label(set_node, scope_node, "scope");
    }

    fn visit_function_call(&mut self, function_call: &FunctionCall) {
        let function_node = self.new_node(&format!(
            "Function Call\n'{}'",
            function_call.name.lexeme()
        ));

        for node in &function_call.arguments {
            let new_node = self.counter;
            node.accept(self);
            self.connect_nodes(function_node, new_node);
        }
    }

    fn visit_list(&mut self, list: &List) {
        let list_node = self.new_node("List");

        for node in &list.expressions {
            let new_node = self.counter;
            node.accept(self);
            self.connect_nodes(list_node, new_node);
        }
    }

    fn visit_atom(&mut self, atom: &Atom) {
        match atom {
            Atom::Boolean(bool) => {
                self.new_node(&format!("Boolean\n'{}'", bool.lexeme()))
            }
            Atom::Number(number) => {
                self.new_node(&format!("Number\n'{}'", number.lexeme()))
            }
            Atom::String(string) => {
                self.new_node(&format!("String\n'{}'", string.lexeme()))
            }
            Atom::Symbol(symbol) => {
                self.new_node(&format!("Symbol\n'{}'", symbol.lexeme()))
            }
            Atom::Nil(_) => self.new_node("Nil"),
        };
    }
}
