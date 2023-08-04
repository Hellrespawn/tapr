#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]
use crate::interpreter::{Arguments, Interpreter};
use crate::node::NodeSource;
use crate::{Environment, Node, NodeData, Result};

use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};

pub struct Boolean;

impl NativeModule for Boolean {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("!", not, "b"),
            (">", gt, "& b"),
            (">=", gte, "& b"),
            ("==", eq, "& b"),
            ("<=", lte, "& b"),
            ("<", lt, "& b"),
            ("!=", ne, "& b"),
            ("or", or, "& v"),
            ("and", and, "& v"),
            ("??", nil_coalesce, "& v"),
        ];

        let mut env = Environment::new();

        function_tuples_to_environment(&mut env, tuples, self.name());

        env
    }

    fn name(&self) -> &'static str {
        "arithmetic"
    }

    fn is_core_module(&self) -> bool {
        true
    }
}

type BinaryOp = fn(Node, Node) -> bool;
type UnaryOp = fn(Node) -> bool;

fn variadic(op: BinaryOp, arguments: Arguments) -> Result<Node> {
    let values = arguments.unwrap_from(0);

    let mut acc = true;

    for window in values.windows(2) {
        let lhs = window[0].clone();
        let rhs = window[1].clone();

        acc = op(lhs, rhs);
    }

    Ok(Node::unknown(NodeData::Bool(acc)))
}

fn binary(op: BinaryOp, arguments: Arguments) -> Result<Node> {
    let lhs = arguments.unwrap(0);
    let rhs = arguments.unwrap(1);

    Ok(Node::unknown(NodeData::Bool(op(lhs, rhs))))
}

fn unary(op: UnaryOp, arguments: Arguments) -> Result<Node> {
    let value = arguments.unwrap(0);

    Ok(Node::unknown(NodeData::Bool(op(value))))
}

pub fn not(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    unary(|v| v.is_falsy(), arguments)
}

pub fn gt(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    if arguments.len() == 2 {
        binary(|lhs, rhs| lhs > rhs, arguments)
    } else {
        variadic(|lhs, rhs| lhs < rhs, arguments)
    }
}

pub fn gte(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    if arguments.len() == 2 {
        binary(|lhs, rhs| lhs >= rhs, arguments)
    } else {
        variadic(|lhs, rhs| lhs <= rhs, arguments)
    }
}

pub fn eq(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    variadic(|lhs, rhs| lhs == rhs, arguments)
}

pub fn lte(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    if arguments.len() == 2 {
        binary(|lhs, rhs| lhs <= rhs, arguments)
    } else {
        variadic(|lhs, rhs| lhs >= rhs, arguments)
    }
}

pub fn lt(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    if arguments.len() == 2 {
        binary(|lhs, rhs| lhs < rhs, arguments)
    } else {
        variadic(|lhs, rhs| lhs > rhs, arguments)
    }
}

pub fn ne(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    variadic(|lhs, rhs| lhs != rhs, arguments)
}

pub fn or(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let values = arguments.unwrap_from(0);
    let last_index = values.len() - 1;

    for (i, argument) in values.into_iter().enumerate() {
        if argument.is_truthy() || i == last_index {
            return Ok(argument);
        }
    }

    unreachable!()
}

pub fn and(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let values = arguments.unwrap_from(0);
    let last_index = values.len() - 1;

    for (i, argument) in values.into_iter().enumerate() {
        if argument.is_falsy() || i == last_index {
            return Ok(argument);
        }
    }

    unreachable!()
}

pub fn nil_coalesce(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    let values = arguments.unwrap_from(0);
    let last_index = values.len() - 1;

    for (i, argument) in values.into_iter().enumerate() {
        if !argument.is_nil() || i == last_index {
            return Ok(argument);
        }
    }

    unreachable!()
}
