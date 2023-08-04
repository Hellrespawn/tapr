#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::unnecessary_wraps)]

use super::{
    function_tuples_to_environment, macro_tuples_to_environment,
    NativeFunctionTuple, NativeMacroTuple, NativeModule,
};
use crate::interpreter::{Arguments, Interpreter};
use crate::node::NodeSource;
use crate::{Environment, Node, NodeData, Result};

pub struct Core;

impl NativeModule for Core {
    fn environment(&self) -> Environment {
        let function_tuples: Vec<NativeFunctionTuple> = vec![
            ("println", println, "& s"),
            ("print", print, "& s"),
            ("is-nil", is_nil, "v"),
        ];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, function_tuples, self.name());

        let macro_tuples: Vec<NativeMacroTuple> =
            vec![("cond", cond, "& pairs")];

        macro_tuples_to_environment(&mut env, macro_tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "core"
    }

    fn is_core_module(&self) -> bool {
        true
    }
}

fn println(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    println!();

    Ok(Node::unknown(NodeData::Nil))
}

fn print(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    for argument in arguments.arguments() {
        print!("{argument}");
    }

    Ok(Node::unknown(NodeData::Nil))
}

fn is_nil(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let argument = arguments.unwrap(0);

    Ok(Node::unknown(NodeData::Bool(argument.is_nil())))
}

fn cond(
    source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let mut arguments = arguments.unwrap_from(0);

    let mut node = if arguments.len() % 2 == 1 {
        arguments.pop().unwrap()
    } else {
        Node::new(source, NodeData::Nil)
    };

    for pair in arguments.rchunks(2) {
        node = Node::new(
            pair[0].source(),
            NodeData::PTuple(vec![
                Node::new(pair[0].source(), NodeData::Symbol("if".to_owned())),
                pair[0].clone(),
                pair[1].clone(),
                node,
            ]),
        );
    }

    Ok(node)
}
