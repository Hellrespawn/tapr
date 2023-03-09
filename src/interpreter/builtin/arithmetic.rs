use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Arguments, Interpreter, Value};
use crate::Result;

type ArithmeticOp = fn(f64, f64) -> f64;

#[allow(clippy::unnecessary_wraps)]
fn arithmetic(op: ArithmeticOp, arguments: Vec<Value>) -> Result<Value> {
    let params = arithmetic_params();

    let arguments = Arguments::new(&params, arguments)?;

    let numbers = arguments.unwrap_numbers();

    let mut iter = numbers.into_iter();

    let mut acc = iter.next().expect("at least one arguments");

    for rhs in iter {
        acc = op(acc, rhs);
    }

    Ok(Value::Number(acc))
}

pub fn add(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    arithmetic(|lhs, rhs| lhs + rhs, arguments)
}

pub fn sub(_intp: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
    arithmetic(|lhs, rhs| lhs - rhs, arguments)
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

pub fn arithmetic_params() -> Parameters {
    let param = Parameter::anonymous(ParameterType::Number, true);

    Parameters::new(vec![param]).expect("arithmetic to have valid params")
}
