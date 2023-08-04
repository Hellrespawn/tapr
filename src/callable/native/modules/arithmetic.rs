#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::needless_pass_by_value)]
use super::{
    function_tuples_to_environment, NativeFunctionTuple, NativeModule,
};
use crate::interpreter::{Arguments, Environment, Interpreter};
use crate::node::NodeSource;
use crate::{Node, NodeData, Result};

pub struct Arithmetic;

impl NativeModule for Arithmetic {
    fn environment(&self) -> Environment {
        let tuples: Vec<NativeFunctionTuple> = vec![
            ("+", add, "n:number & m:number"),
            ("-", subtract, "n:number & m:number"),
            ("/", divide, "n:number & m:number"),
            ("*", multiply, "n:number & m:number"),
            ("%", modulus, "n:number m:number"),
            ("++", increment, "n:number"),
            ("--", decrement, "n:number"),
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

type BinaryOp = fn(f64, f64) -> f64;
type UnaryOp = fn(f64) -> f64;

fn variadic(op: BinaryOp, arguments: Arguments) -> Result<Node> {
    let numbers = arguments.unwrap_numbers();

    let mut iter = numbers.into_iter();

    let mut acc = iter.next().expect("at least two arguments");

    for rhs in iter {
        acc = op(acc, rhs);
    }

    Ok(Node::unknown(NodeData::Number(acc)))
}

fn binary(op: BinaryOp, arguments: Arguments) -> Result<Node> {
    let [lhs, rhs] = arguments.unwrap_numbers()[..] else {
        panic!()
    };

    Ok(Node::unknown(NodeData::Number(op(lhs, rhs))))
}

fn unary(op: UnaryOp, arguments: Arguments) -> Result<Node> {
    let number = arguments.unwrap_numbers()[0];

    Ok(Node::unknown(NodeData::Number(op(number))))
}

pub fn add(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    variadic(|lhs, rhs| lhs + rhs, arguments)
}

pub fn subtract(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    variadic(|lhs, rhs| lhs - rhs, arguments)
}

pub fn multiply(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    variadic(|lhs, rhs| lhs * rhs, arguments)
}

pub fn divide(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    variadic(|lhs, rhs| lhs / rhs, arguments)
}

pub fn modulus(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    binary(|lhs, rhs| lhs % rhs, arguments)
}

pub fn increment(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    unary(|n| n + 1.0, arguments)
}

pub fn decrement(
    _source: NodeSource,
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Node> {
    unary(|n| n - 1.0, arguments)
}
