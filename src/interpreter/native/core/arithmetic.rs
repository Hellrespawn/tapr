#![allow(clippy::unnecessary_wraps)]
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

type BinaryOp = fn(f64, f64) -> f64;
type UnaryOp = fn(f64) -> f64;

fn variadic(op: BinaryOp, arguments: &Arguments) -> Result<Value> {
    let numbers = arguments.unwrap_numbers();

    let mut iter = numbers.into_iter();

    let mut acc = iter.next().expect("at least two arguments");

    for rhs in iter {
        acc = op(acc, rhs);
    }

    Ok(Value::Number(acc))
}

fn binary(op: BinaryOp, arguments: &Arguments) -> Result<Value> {
    let [lhs, rhs] = arguments.unwrap_numbers()[..] else { panic!() };

    Ok(Value::Number(op(lhs, rhs)))
}

fn unary(op: UnaryOp, arguments: &Arguments) -> Result<Value> {
    let number = arguments.unwrap_numbers()[0];

    Ok(Value::Number(op(number)))
}

pub fn add(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs + rhs, arguments)
}

pub fn subtract(
    _intp: &mut Interpreter,
    arguments: &Arguments,
) -> Result<Value> {
    variadic(|lhs, rhs| lhs - rhs, arguments)
}

pub fn multiply(
    _intp: &mut Interpreter,
    arguments: &Arguments,
) -> Result<Value> {
    variadic(|lhs, rhs| lhs * rhs, arguments)
}

pub fn divide(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs / rhs, arguments)
}

pub fn modulus(
    _intp: &mut Interpreter,
    arguments: &Arguments,
) -> Result<Value> {
    binary(|lhs, rhs| lhs % rhs, arguments)
}

pub fn increment(
    _intp: &mut Interpreter,
    arguments: &Arguments,
) -> Result<Value> {
    unary(|n| n + 1.0, arguments)
}

pub fn decrement(
    _intp: &mut Interpreter,
    arguments: &Arguments,
) -> Result<Value> {
    unary(|n| n - 1.0, arguments)
}
