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
            rankdir=\"TB\";\n  \
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
            let new_node = self.counter;
            node.accept(self);

            self.connect_nodes(parent_node, new_node);
        }
    }
}

impl Visitor<()> for GraphVisitor {
    fn visit_define(&mut self, define: &Define) {
        todo!()
    }

    fn visit_if(&mut self, if_expr: &If) {
        todo!()
    }

    fn visit_while(&mut self, while_expr: &While) {
        todo!()
    }

    fn visit_lambda(&mut self, lambda: &Lambda) {
        todo!()
    }

    fn visit_call(&mut self, call: &Call) {
        todo!()
    }

    fn visit_datum(&mut self, atom: &Datum) {
        todo!()
    }
    // fn visit_program(&mut self, program: &Program) {
    //     let program_node = self.new_node("Program");

    //     self.accept_and_connect(program_node, &program.expression);
    // }

    // fn visit_if_expression(&mut self, if_expression: &IfExpression) {
    //     let if_node = self.new_node("If");

    //     self.accept_and_connect_with_label(
    //         if_node,
    //         &if_expression.condition,
    //         "condition",
    //     );

    //     self.accept_and_connect_with_label(
    //         if_node,
    //         &if_expression.then_branch,
    //         "then",
    //     );

    //     if let Some(else_branch) = &if_expression.else_branch {
    //         self.accept_and_connect_with_label(if_node, else_branch, "else");
    //     }
    // }

    // fn visit_while_expression(&mut self, while_expression: &WhileExpression) {
    //     let while_node = self.new_node("While");

    //     self.accept_and_connect_with_label(
    //         while_node,
    //         &while_expression.condition,
    //         "condition",
    //     );

    //     self.accept_and_connect_with_label(
    //         while_node,
    //         &while_expression.expression,
    //         "expression",
    //     );
    // }

    // fn visit_set_expression(&mut self, set_expression: &SetExpression) {
    //     let set_node =
    //         self.new_node(&format!("Set\n'{}'", set_expression.name.lexeme()));

    //     self.accept_and_connect(set_node, &set_expression.expression);
    // }

    // fn visit_function_call(&mut self, function_call: &FunctionCall) {
    //     let function_node = self.new_node(&format!(
    //         "Function Call\n'{}'",
    //         function_call.name.lexeme()
    //     ));

    //     self.accept_and_connect_many(function_node, &function_call.arguments);
    // }

    // fn visit_function_definition(
    //     &mut self,
    //     function_definition: &FunctionDefinition,
    // ) {
    //     let function_node = self
    //         .new_node(&format!("def\n'{}'", function_definition.name.lexeme()));

    //     for param in &function_definition.parameters {
    //         let param_node = self.new_node(param.lexeme());

    //         self.connect_nodes_with_label(
    //             function_node,
    //             param_node,
    //             "parameter",
    //         );
    //     }

    //     self.accept_and_connect_with_label(
    //         function_node,
    //         &function_definition.expression,
    //         "expression",
    //     );
    // }

    // fn visit_list(&mut self, list: &List) {
    //     let list_node = self.new_node("List");

    //     self.accept_and_connect_many(list_node, &list.expressions);
    // }

    // fn visit_atom(&mut self, atom: &Atom) {
    //     match atom {
    //         Atom::Boolean(bool) => {
    //             self.new_node(&format!("Boolean\n'{}'", bool.lexeme()))
    //         }
    //         Atom::Number(number) => {
    //             self.new_node(&format!("Number\n'{}'", number.lexeme()))
    //         }
    //         Atom::String(string) => {
    //             self.new_node(&format!("String\n'{}'", string.lexeme()))
    //         }
    //         Atom::Symbol(symbol) => {
    //             self.new_node(&format!("Symbol\n'{}'", symbol.lexeme()))
    //         }
    //         Atom::Nil(_) => self.new_node("Nil"),
    //     };
    // }
}
