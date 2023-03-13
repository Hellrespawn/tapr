use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

type ArithmeticOp = fn(f64, f64) -> f64;
type UnaryOp = fn(f64) -> f64;

fn arithmetic(op: ArithmeticOp, arguments: Vec<Value>) -> Result<Value> {
    let params = arithmetic_params();

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
    arithmetic(|lhs, rhs| lhs + rhs, arguments)
}

pub fn sub(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    if arguments.len() == 1 {
        unary(|n| -n, arguments)
    } else {
        arithmetic(|lhs, rhs| lhs - rhs, arguments)
    }
}

pub fn mul(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    arithmetic(|lhs, rhs| lhs * rhs, arguments)
}

pub fn div(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    arithmetic(|lhs, rhs| lhs / rhs, arguments)
}

pub fn modulus(
    _intp: &mut Interpreter,
    arguments: Vec<Value>,
) -> Result<Value> {
    arithmetic(|lhs, rhs| lhs % rhs, arguments)
}

pub fn unary_params() -> Parameters {
    Parameters::new(vec![Parameter::anonymous(ParameterType::Number, false)])
        .expect("unary to have valid params")
}

pub fn arithmetic_params() -> Parameters {
    Parameters::new(vec![
        Parameter::anonymous(ParameterType::Number, false),
        Parameter::anonymous(ParameterType::Number, true),
    ])
    .expect("arithmetic to have valid params")
}
