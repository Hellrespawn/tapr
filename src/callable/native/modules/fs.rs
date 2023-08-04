#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::interpreter::{Arguments, Interpreter};
use crate::node::NodeSource;
use crate::{Environment, Node, NodeData, Result};

pub struct Fs;

impl NativeModule for Fs {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("read_to_string", read_to_string, "path:string"),
            ("write", write, "path:string body:string"),
        ];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "fs"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn read_to_string(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let path = arguments.unwrap_string(0);

    Ok(Node::unknown(NodeData::String(std::fs::read_to_string(
        path,
    )?)))
}

fn write(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let path = arguments.unwrap_string(0);
    let body = arguments.unwrap_string(1);

    std::fs::write(path, body)?;

    Ok(Node::unknown(NodeData::Nil))
}
