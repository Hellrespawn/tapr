#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::error::ErrorKind;
use crate::interpreter::{Arguments, Interpreter};
use crate::node::NodeSource;
use crate::{Environment, Node, NodeData, ParameterType, Result};

pub struct StringModule;

impl NativeModule for StringModule {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("len", len, "s:string"),
            ("join", join, "separator:string l:list"),
            ("join-not-nil", join_not_nil, "separator:string l:list"),
            ("trim", trim, "s:string"),
            ("split", split, "separator:string string:string"),
        ];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "string"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

type UnaryOp = fn(&str) -> String;

fn unary(op: UnaryOp, arguments: Arguments) -> Result<Node> {
    let string = arguments.unwrap_string(0);

    Ok(Node::unknown(NodeData::String(op(&string))))
}

fn len(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let string = arguments.unwrap_string(0);

    #[allow(clippy::cast_precision_loss)]
    Ok(Node::unknown(NodeData::Number(string.len() as f64)))
}

fn join(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let separator = arguments.unwrap_string(0);
    let nodes = arguments.unwrap_list(1);

    let strings = nodes
        .into_iter()
        .map(|node| {
            if let Some(string) = node.as_string() {
                Ok(string.to_owned())
            } else {
                Err(ErrorKind::InvalidNodeArgument {
                    expected: vec![ParameterType::String],
                    actual: node,
                }
                .into())
            }
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Node::unknown(NodeData::String(strings.join(&separator))))
}

fn join_not_nil(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let separator = arguments.unwrap_string(0);
    let values = arguments.unwrap_list(1);

    let strings = values
        .into_iter()
        .filter_map(|value| {
            if value.is_nil() {
                None
            } else if let Some(string) = value.as_string() {
                Some(Ok(string.to_owned()))
            } else {
                Some(Err(ErrorKind::InvalidNodeArgument {
                    expected: vec![ParameterType::String],
                    actual: value,
                }
                .into()))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    if strings.is_empty() {
        Ok(Node::unknown(NodeData::Nil))
    } else {
        Ok(Node::unknown(NodeData::String(strings.join(&separator))))
    }
}

fn trim(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    unary(|s| s.trim().to_owned(), arguments)
}

fn split(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let separator = arguments.unwrap_string(0);
    let string = arguments.unwrap_string(1);

    let values = string
        .split(&separator)
        .map(|s| Node::unknown(NodeData::String(s.to_owned())))
        .collect::<Vec<_>>();

    Ok(Node::unknown(NodeData::BTuple(values)))
}
