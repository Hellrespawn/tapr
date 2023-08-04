#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]

use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::interpreter::{Arguments, Interpreter};
use crate::node::NodeSource;
use crate::{Environment, Node, NodeData, Result};

pub struct List;

impl NativeModule for List {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("head", head, "l:list"),
            ("tail", tail, "l:list"),
            ("push", push, "l:list"),
            ("filter", filter, "f:function l:list"),
            ("map", map, "f:function l:list"),
            ("reduce", reduce, "f:function init l:list"),
        ];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "list"
    }

    fn is_core_module(&self) -> bool {
        false
    }
}

fn head(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let list = arguments.unwrap_list(0);

    Ok(list
        .into_iter()
        .next()
        .unwrap_or_else(|| Node::unknown(NodeData::Nil)))
}

fn tail(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let list = arguments
        .unwrap_list(0)
        .get(1..)
        .map(Vec::from)
        .unwrap_or_default();

    Ok(Node::unknown(NodeData::BTuple(list)))
}

fn push(
    _source: NodeSource,
    _: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let list = arguments.unwrap_list(0);

    let values = arguments.arguments()[1..].to_owned();

    let output = [list, values].into_iter().flatten().collect();

    Ok(Node::unknown(NodeData::BTuple(output)))
}

fn reduce(
    source: NodeSource,
    intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let function = arguments.unwrap_function(0);
    let init = arguments.unwrap(1);
    let input = arguments.unwrap_list(2);

    let mut output = init;

    for value in input {
        output = function.call(
            source.clone(),
            intp,
            Arguments::from_nodes(&function.parameters(), vec![output, value])?,
        )?;
    }

    Ok(output)
}

fn filter(
    source: NodeSource,
    intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let function = arguments.unwrap_function(0);
    let values = arguments.unwrap_list(1);

    let mut output = Vec::new();

    for value in values {
        let is_truthy = function
            .call(
                source.clone(),
                intp,
                Arguments::from_nodes(
                    &function.parameters(),
                    vec![value.clone()],
                )?,
            )?
            .is_truthy();

        if is_truthy {
            output.push(value);
        }
    }

    Ok(Node::unknown(NodeData::BTuple(output)))
}

fn map(
    source: NodeSource,
    intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let function = arguments.unwrap_function(0);
    let values = arguments.unwrap_list(1);

    let output = values
        .into_iter()
        .map(|v| {
            function.call(
                source.clone(),
                intp,
                Arguments::from_nodes(&function.parameters(), vec![v])?,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Node::unknown(NodeData::BTuple(output)))
}
