use crate::interpreter::parameters::{Parameter, ParameterType, Parameters};
use crate::interpreter::{Interpreter, Value};
use crate::parser::ast::Expression;
use crate::Result;

type ArithmeticOp = fn(Value, Value) -> Result<Value>;

fn arithmetic(
    intp: &mut Interpreter,
    op: ArithmeticOp,
    argument_nodes: &[Expression],
) -> Result<Value> {
    let arguments =
        arithmetic_params().evaluate_arguments(intp, argument_nodes)?;

    let mut iter = arguments.into_iter();

    let mut acc = iter.next().expect("at least one arguments");

    for rhs in iter {
        acc = op(acc, rhs)
            .expect("Parameters should have been checked as numbers first.");
    }

    Ok(acc)
}

pub fn add(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    arithmetic(intp, |lhs, rhs| lhs + rhs, argument_nodes)
}

pub fn sub(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    arithmetic(intp, |lhs, rhs| lhs - rhs, argument_nodes)
}

pub fn mul(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    arithmetic(intp, |lhs, rhs| lhs * rhs, argument_nodes)
}

pub fn div(
    intp: &mut Interpreter,
    argument_nodes: &[Expression],
) -> Result<Value> {
    arithmetic(intp, |lhs, rhs| lhs / rhs, argument_nodes)
}

pub fn arithmetic_params() -> Parameters {
    let param = Parameter::new(
        "_arithmetic".to_owned(),
        vec![ParameterType::Number],
        true,
    );

    Parameters::new(vec![param]).expect("arithmetic to have valid params")
}
