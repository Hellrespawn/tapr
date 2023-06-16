#![allow(clippy::unnecessary_wraps)]
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

type BinaryOp = fn(Value, Value) -> bool;
type UnaryOp = fn(Value) -> bool;

fn variadic(op: BinaryOp, arguments: &Arguments) -> Result<Value> {
    let values = arguments.unwrap_from(0);

    let mut acc = true;

    for window in values.windows(2) {
        let lhs = window[0].clone();
        let rhs = window[1].clone();

        acc = op(lhs, rhs);
    }

    Ok(Value::Boolean(acc))
}

// fn binary(op: BinaryOp, arguments: &Arguments) -> Result<Value> {
//     let values = arguments.unwrap_from(0);

//     let lhs = values[0].clone();
//     let rhs = values[1].clone();

//     Ok(Value::Boolean(op(lhs, rhs)))
// }

fn unary(op: UnaryOp, arguments: &Arguments) -> Result<Value> {
    let value = arguments.unwrap(0);

    Ok(Value::Boolean(op(value)))
}

pub fn not(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    unary(|v| v.is_falsy(), arguments)
}

pub fn gt(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs < rhs, arguments)
}

pub fn gte(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs <= rhs, arguments)
}

pub fn eq(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs == rhs, arguments)
}

pub fn lte(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs >= rhs, arguments)
}

pub fn lt(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs > rhs, arguments)
}

pub fn ne(_intp: &mut Interpreter, arguments: &Arguments) -> Result<Value> {
    variadic(|lhs, rhs| lhs != rhs, arguments)
}
