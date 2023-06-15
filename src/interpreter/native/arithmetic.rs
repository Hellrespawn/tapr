use crate::interpreter::parameters::{Parameter, Parameters};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

type BinaryOp = fn(f64, f64) -> f64;
type UnaryOp = fn(f64) -> f64;

fn arbitrary(op: BinaryOp, arguments: Arguments) -> Result<Value> {
    let numbers = arguments.unwrap_numbers();

    let mut iter = numbers.into_iter();

    let mut acc = iter.next().expect("at least two arguments");

    for rhs in iter {
        acc = op(acc, rhs);
    }

    Ok(Value::Number(acc))
}

fn unary(op: UnaryOp, arguments: Arguments) -> Result<Value> {
    let number = arguments.unwrap_numbers()[0];

    Ok(Value::Number(op(number)))
}

pub fn add(_intp: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs + rhs, arguments)
}

pub fn subtract(
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs - rhs, arguments)
}

pub fn multiply(
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs * rhs, arguments)
}

pub fn divide(_intp: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs / rhs, arguments)
}

pub fn modulus(_intp: &mut Interpreter, arguments: Arguments) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs % rhs, arguments)
}

pub fn increment(
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Value> {
    unary(|n| n + 1.0, arguments)
}

pub fn decrement(
    _intp: &mut Interpreter,
    arguments: Arguments,
) -> Result<Value> {
    unary(|n| n - 1.0, arguments)
}

pub fn unary_params() -> Parameters {
    Parameter::new("operand".to_owned()).number().into()
}

pub fn arbitrary_params() -> Parameters {
    Parameters::new(vec![
        Parameter::new("operand".to_owned()).number(),
        Parameter::new("operands".to_owned()).number().rest(),
    ])
    .expect("arithmetic to have valid params")
}
