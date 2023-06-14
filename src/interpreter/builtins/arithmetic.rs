use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

type BinaryOp = fn(f64, f64) -> f64;
type UnaryOp = fn(f64) -> f64;

fn arbitrary(op: BinaryOp, arguments: Vec<Value>) -> Result<Value> {
    let params = arbitrary_params();

    let arguments = Arguments::new(&params, arguments)?;

    let numbers = arguments.unwrap_numbers();

    let mut iter = numbers.into_iter();

    let mut acc = iter.next().expect("at least two arguments");

    for rhs in iter {
        acc = op(acc, rhs);
    }

    Ok(Value::Number(acc))
}

fn unary(op: UnaryOp, arguments: Vec<Value>) -> Result<Value> {
    let params = unary_params();
    let arguments = Arguments::new(&params, arguments)?;

    let number = arguments.unwrap_numbers()[0];

    Ok(Value::Number(op(number)))
}

pub fn add(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs + rhs, arguments)
}

pub fn subtract(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    if arguments.len() == 1 {
        unary(|n| -n, arguments)
    } else {
        arbitrary(|lhs, rhs| lhs - rhs, arguments)
    }
}

pub fn multiply(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs * rhs, arguments)
}

pub fn divide(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs / rhs, arguments)
}

pub fn modulus(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    arbitrary(|lhs, rhs| lhs % rhs, arguments)
}

pub fn increment(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    unary(|n| n + 1.0, arguments)
}

pub fn decrement(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    unary(|n| n - 1.0, arguments)
}

pub fn unary_params() -> Parameters {
    Parameters::new(vec![Parameter::anonymous(
        vec![ParameterType::Number],
        false,
    )])
    .expect("unary to have valid params")
}

pub fn arbitrary_params() -> Parameters {
    Parameters::new(vec![
        Parameter::anonymous(vec![ParameterType::Number], false),
        Parameter::anonymous(vec![ParameterType::Number], true),
    ])
    .expect("arithmetic to have valid params")
}
